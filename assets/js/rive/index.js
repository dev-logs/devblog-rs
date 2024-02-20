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

class RiveController {
    constructor(rivFileUrl, canvas) {
        this._rive = RiveApp.currentInstance.rive
        if (!this._rive) {
            throw Error("The rive has not been initialize, call await RiveApp.init() first")
        }

        this._canvas = canvas
        this._renderer = this._rive.makeRenderer(canvas)
        this._rivFileUrl = rivFileUrl
        this._fits = {}
        this._alignments = {}
        this._stateMachines = {}
        this._cleanupTasks = []
        this._listeners = {}
    }

    async getRivFile() {
        if (!this._rivFile) {
            const rivRequest = await fetch(new Request(this._rivFileUrl))
            const bytes = await rivRequest.arrayBuffer()
            this._rifFile = await this._rive.load(new Uint8Array(bytes))
        }

        return this._rifFile
    }

    async getArtBoard() {
        if (!this._artboard) {
            const file = await this.getRivFile()
            this._artboard = file.artboardByName('thumb')
        }

        return this._artboard
    }

    setFit(state, value) {
        this._fits[state] = value
    }

    setAlignment(state, value) {
        this._alignments[state] = value
    }

    getFit(state) {
        return this._fits[state] || this._rive.Fit.contain
    }

    getAlignment(state) {
        return this._alignments[state] || this._rive.Alignment.center
    }

    async registerListeners(state) {
        const rive = this._rive
        const fit = this.getFit(state)
        const alignment = this.getAlignment(state)
        const canvas = this._canvas
        const artboard = await this.getArtBoard()
        const stateMachine = await this.getStateMachine(state)

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

    async getStateMachine(name) {
        if (!name) return null

        if (!this._stateMachines) this._stateMachines = {}

        if (!this._stateMachines[name]) {
            const artboard = await this.getArtBoard()
            this._stateMachines[name] = new this._rive.StateMachineInstance(
                artboard.stateMachineByName(name),
                artboard
            )
        }

        return this._stateMachines[name]
    }

    on(state, event, callback) {
        if (!this._listeners[state]) this._listeners[state] = {}
        const stateListeners = this._listeners[state]

        if (!stateListeners[event]) stateListeners[event] = []
        const listeners = stateListeners[event]

        listeners.push(callback)

        this._cleanupTasks.push(() => {
            if (!stateListeners[event]) return

            const index = stateListeners[event].indexOf(callback)
            if (index > -1) {
                listeners.splice(index, 1)
            }
        })
    }

    dispatchEvent(state, event) {
        const stateListeners = this._listeners[state]
        if (stateListeners && stateListeners[event.name]) {
            stateListeners[event.name].forEach((it) => it(event))
        }
    }

    async dispose() {
        this._cleanupTasks.forEach((it) => it())
    }

    async render(state) {
        if (this._isRendering) return
        this._isRendering = true

        console.log('rendering thumb thumb')

        const renderer = this._renderer
        const artboard = await this.getArtBoard()
        const stateMachine = await this.getStateMachine(state)
        await this.registerListeners(state)

        RiveApp.currentInstance.requestRenderLoop(({time, deltaTime}) => {
            const elapsedTimeSec = deltaTime / 1000

            const numFiredEvents = stateMachine.reportedEventCount()
            for (let i = 0; i < numFiredEvents; i++) {
                const event = stateMachine.reportedEventAt(i)
                this.dispatchEvent(state, event)
            }

            renderer.clear()
            stateMachine.advance(elapsedTimeSec)
            // animation.advance(elapsedTimeSec)
            // animation.apply(1)
            artboard.advance(elapsedTimeSec)
            renderer.save()
            renderer.align(
                this.getFit(state),
                this.getAlignment(state),
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

class ThumbUpRiveComponent extends HTMLElement {
    async connectedCallback() {
        const {width, height} = this.getBoundingClientRect()
        this.canvas = document.createElement('canvas')
        console.log(width, height)
        this.canvas.width = width
        this.canvas.height = height
        this.appendChild(this.canvas)

        await RiveApp.getInstance()
        const controller = new RiveThumbUpController({
            canvas: this.canvas,
            numberOfLikes: this.numberOfLikes
        })

        await controller.render()
    }
}

customElements.define('thumb-up', ThumbUpRiveComponent)

class RiveThumbUpController extends RiveController {
    state = 'thumb_up'

    constructor({canvas, numberOfLikes}) {
        super('/assets/riv/rive.riv', canvas)
        this._numberOfLikes = numberOfLikes
    }

    async updateLikes(value) {
        const textRunLikes = (await this.getArtBoard()).textRun("text_run_likes")
        this._numberOfLikes = value
        textRunLikes.text = (this._numberOfLikes).toString()
    }

    async render() {
        if (this._isRendering) return

        this.on(this.state, 'LikeEvent', async(event) => {
            callback(this._numberOfLikes)
        })

        return super.render(this.state)
    }
}
