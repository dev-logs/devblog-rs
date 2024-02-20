const RIVE_VERSION = '2.7.0'

class RiveApp {
    static _instance
    constructor(rive) {
        this._rive = rive
    }

    get rive() {
        return this._rive
    }

    static async getInstance() {
        if (RiveApp._instance) {
            return RiveApp._instance
        }

        const {default: RiveCanvas} = await import((`https://unpkg.com/@rive-app/canvas-advanced@${RIVE_VERSION}`))
        const rive = await RiveCanvas({
            locateFile: (_) => `https://unpkg.com/@rive-app/canvas-advanced@${RIVE_VERSION}/rive.wasm`
        })

        RiveApp._instance = new RiveApp(rive)
        return RiveApp._instance
    }

    static get currentInstance() {
        return RiveApp._instance
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
    constructor() {
        super()
        this._cleanupTasks = []
    }

    async connectedCallback() {
        await RiveApp.getInstance()
        this._rive = RiveApp.currentInstance.rive
        if (!this._rive) {
            throw Error("The rive has not been initialize, call await RiveApp.init() first")
        }

        const {width, height} = this.getBoundingClientRect()
        const canvas = document.createElement('canvas')
        canvas.width = width
        canvas.height = height
        this.appendChild(canvas)
        this._canvas = canvas

        this._renderer = this._rive.makeRenderer(canvas)
        const rivRequest = await fetch(new Request(this.rivFileUrl))
        const bytes = await rivRequest.arrayBuffer()
        this._rifFile = await this._rive.load(new Uint8Array(bytes))
        this._artboard = this._rifFile.artboardByName(this.artboardName)
        this._stateMachine = new this._rive.StateMachineInstance(
            this._artboard.stateMachineByName(this.state),
            this._artboard
        )

        await this.render()
    }

    set rivFileUrl(value) {
        this.setAttribute('rivFileUrl', value)
    }

    get rivFileUrl() {
        return this.getAttribute('rivFileUrl')
    }

    set state(value) {
        this.setAttribute('state', value)
    }

    get state() {
        return this.getAttribute('state')
    }

    set artboardName(value) {
        this.setAttribute('artboardName', value)
    }

    get artboardName() {
        return this.getAttribute('artboardName')
    }

    set fit(value) {
        this.setAttribute('fit', Object.keys(this._rive.Alignment).find((key) => this._rive.Fit[key] === value))
    }

    set alignment(value) {
        this.setAttribute('alignment', Object.keys(this._rive.Alignment).find((key) => this._rive.Alignment[key] === value))
    }

    get fit() {
        const fitAttr = this.getAttribute('fit')
        if (!fitAttr) {
            this.setAttribute('fit', 'contain')
        }

        const name = this.getAttribute('fit')
        return this._rive.Fit[name]
    }

    get alignment() {
        const alignmentAttr = this.getAttribute('alignment')
        if (!alignmentAttr) {
            this.setAttribute('alignment', 'center')
        }

        const name = this.getAttribute('alignment')
        return this._rive.Alignment[name]
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

    async render() {
        if (this._isRendering) return
        this._isRendering = true

        const renderer = this._renderer
        const artboard = this._artboard
        const stateMachine = this._stateMachine
        await this.registerListeners(stateMachine)

        RiveApp.currentInstance.requestRenderLoop(({time, deltaTime}) => {
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
            // animation.advance(elapsedTimeSec)
            // animation.apply(1)
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

    async connectedCallback() {
        await super.connectedCallback()
        this.likeCount = this.getAttribute('likeCount')
    }

    set likeCount(value) {
        this.setAttribute('likeCount', value)
        if (this._stateMachine) {
            const textRun = this._artboard.textRun("text_run_likes")
            textRun.text = this.likeCount.toString()
        }
    }

    get likeCount() {
        let n = Number(this.getAttribute('likeCount'))
        if (isNaN(n)) n = 0

        return n
    }
}

customElements.define('rive-component', RiveComponent)
customElements.define('thumb-up', ThumbUpRiveComponent)
