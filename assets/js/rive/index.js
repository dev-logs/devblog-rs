const RIVE_VERSION = '2.7.0'

/*
* Represent for the Rive instance
* it will loaded as singleton in the first time, and later use will always using the same rive instance
* load library at @rive-app manually because it is a js module (.mjs) so that we need to load it internally
* */
class RiveApp {
    constructor(rive) {
        this._rive = rive
    }

    get rive() {
        return this._rive
    }

    static async init() {
        const {default: RiveCanvas} = await import((`https://unpkg.com/@rive-app/canvas-advanced@${RIVE_VERSION}`))
        const rive = await RiveCanvas({
            locateFile: (_) => `https://unpkg.com/@rive-app/canvas-advanced@${RIVE_VERSION}/rive.wasm`
        })

        const instance = new RiveApp(rive)
        return instance
    }

    requestRenderLoop(callback) {
        let lastTime = 0

        const renderLoop = (time) => {
            if (!lastTime) {
                lastTime = time
            }

            const deltaTime = time - lastTime
            lastTime = time
            callback({time, deltaTime})
            this.rive.requestAnimationFrame(renderLoop)
        }

        this.rive.requestAnimationFrame(renderLoop)
    }
}

class RiveComponent extends HTMLElement {
    static observedAttributes = ["fit", "alignment", "text", "value"];

    constructor() {
        super()
        this._cleanupTasks = []
    }

    async connectedCallback() {
        this._riveApp = await RiveApp.init()
        this._rive = this._riveApp.rive
        const {width, height} = this.getBoundingClientRect()
        const canvas = document.createElement('canvas')
        canvas.width = width
        canvas.height = height
        this.appendChild(canvas)
        this._canvas = canvas

        this._renderer = this._rive.makeRenderer(canvas)
        const fetchRiveFileResponse = await fetch(new Request(this.rivFileUrl))
        const bytes = await fetchRiveFileResponse.arrayBuffer()
        this._rivFile = await this._rive.load(new Uint8Array(bytes))
        this._artboard = this._rivFile.artboardByName(this.artboardName)
        this._stateMachine = new this._rive.StateMachineInstance(
            this._artboard.stateMachineByName(this.state),
            this._artboard
        )

        await this.render()
    }

    disconnectedCallback() {
        console.log('rive-debug', `disconnectedCallback ${this.constructor.name}`)
        this._cleanupTasks.forEach((it) => it())

        this._renderer.delete();
        this._rivFile.delete();
        this._artboard.delete();
        this._stateMachine.delete();
    }

    set rivFileUrl(value) {
        this._rivFileUrl = value
    }

    get rivFileUrl() {
        return this._rivFileUrl
    }

    set state(value) {
        this._state = value
    }

    get state() {
        return this._state
    }

    set artboardName(value) {
        this._artboardName = value
    }

    get artboardName() {
        return this._artboardName
    }

    set fit(value) {
        this._fit = value
    }

    set alignment(value) {
        this._alignment = value
    }

    get fit() {
        if (!this._fit) {
            this._fit = 'contain'
        }

        return this._rive.Fit[this._fit]
    }

    get alignment() {
        if (!this._alignment) {
            this._alignment = 'center'
        }

        return this._rive.Alignment[this._alignment]
    }

    async registerListeners() {
        const rive = this._rive
        const fit = this.fit
        const alignment = this.alignment
        const canvas = this._canvas
        const artboard = this._artboard
        const stateMachine = this._stateMachine

        const mouseCallback = (event) => {
            const boundingRect = event.currentTarget.getBoundingClientRect()

            const canvasX = event.clientX - boundingRect.left
            const canvasY = event.clientY - boundingRect.top
            const forwardMatrix = this._rive.computeAlignment(
                fit,
                alignment,
                {
                    minX: 0,
                    minY: 0,
                    maxX: boundingRect.width,
                    maxY: boundingRect.height
                },
                artboard.bounds
            )

            let invertedMatrix = new rive.Mat2D()
            forwardMatrix.invert(invertedMatrix)
            const canvasCoordinatesVector = new rive.Vec2D(canvasX, canvasY)
            const transformedVector = rive.mapXY(invertedMatrix, canvasCoordinatesVector)

            const transformedX = transformedVector.x()
            const transformedY = transformedVector.y()

            switch (event.type) {
                case "mousemove": {
                    stateMachine.pointerMove(transformedX, transformedY)
                    break
                }
                case "mousedown": {
                    stateMachine.pointerDown(transformedX, transformedY)
                    break
                }
                case "mouseup": {
                    stateMachine.pointerUp(transformedX, transformedY)
                    break
                }

                default:
            }
        }

        const callback = mouseCallback.bind(this)
        canvas.addEventListener("mousemove", callback)
        canvas.addEventListener("mousedown", callback)
        canvas.addEventListener("mouseup", callback)

        this._cleanupTasks.push(() => {
            canvas.removeEventListener("mousemove", callback)
            canvas.removeEventListener("mousedown", callback)
            canvas.removeEventListener("mouseup", callback)
        })
    }

    attributeChangedCallback(name, oldValue, newValue) {
        console.log(`Attribute ${name} has changed.`);
        this[name] = newValue
    }

    async render() {
        if (this._isRendering) return
        this._isRendering = true

        const renderer = this._renderer
        const artboard = this._artboard
        const stateMachine = this._stateMachine
        await this.registerListeners(stateMachine)

        this._riveApp.requestRenderLoop(({time, deltaTime}) => {
            const elapsedTimeSec = deltaTime / 1000

            const numFiredEvents = stateMachine.reportedEventCount()
            for (let i = 0; i < numFiredEvents; i++) {
                const event = stateMachine.reportedEventAt(i)
                this.dispatchEvent(new CustomEvent(event.name, {
                    detail: {target: this},
                    bubbles: true,
                    cancelable: true
                }))
            }

            renderer.clear()
            stateMachine.advance(elapsedTimeSec)
            artboard.advance(elapsedTimeSec)
            renderer.save()
            renderer.align(
                this.fit,
                this.alignment,
                {
                    minX: 0,
                    minY: 0,
                    maxX: this._canvas.width,
                    maxY: this._canvas.height
                },
                artboard.bounds
            )

            artboard.draw(renderer)
            renderer.restore()
        })
    }
}

class ThumbUpRiveComponent extends RiveComponent {
    constructor() {
        super()

        this.state = 'thumb_up'
        this.artboardName = 'thumb'
        this.rivFileUrl = '/assets/riv/rive.riv'
    }
}

class RiveText extends RiveComponent {
    static observedAttributes = ["fit", "alignment", "text", "value"]

    constructor() {
        super()
        this.state = 'controller'
        this.rivFileUrl = '/assets/riv/rive.riv'
        this.artboardName = 'text'
    }

    attributeChangedCallback(name, oldValue, newValue) {
        super.attributeChangedCallback(name, oldValue, newValue);
    }

    set text(value) {
        this._text = value
        if (this._stateMachine) {
            const textRun = this._artboard.textRun("text_run")
            textRun.text = this.text
            const triggerSetTextAnimation = this._stateMachine.input(0).asTrigger()
            triggerSetTextAnimation.fire()
        }
    }

    get text() {
        return this._text
    }
}

class RiveEmojiFaceLove extends RiveComponent {
    constructor() {
        super()
        this.state = 'controller'
        this.rivFileUrl = '/assets/riv/rive.riv'
        this.artboardName = 'love'
    }
}


customElements.define('rive-component', RiveComponent)
customElements.define('rive-thumb-up', ThumbUpRiveComponent)
customElements.define('rive-text', RiveText)
customElements.define('rive-emoji-face-love', RiveEmojiFaceLove)
