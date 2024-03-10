import * as three from 'https://unpkg.com/three@0.162.0/build/three.module.js'

class StarSky3d extends HTMLElement {
    constructor() {
        super()
        this._scene = new three.Scene({alpha: true})

        this._camera = new three.PerspectiveCamera(
            60,
            window.innerWidth / window.innerHeight,
            1,
            1000
        )

        this._camera.position.z = 1
        this._camera.rotation.z = Math.PI / 2

        this._renderer = new three.WebGLRenderer()
        this._renderer.setClearColor( 0x050301, 1)
        this.style.position = 'relative'

        this._size = this.getBoundingClientRect()
        this._responsiveNumber = (this._size.width + this._size.height) / 2890
        this._renderer.setSize(this._size.width, this._size.height)
        this._renderer.domElement.style.position = 'absolute'
        this._renderer.domElement.style.top = '0'
        this._renderer.domElement.style.left = '0'
        this._renderer.domElement.style.width = '100%'
        this._renderer.domElement.style.height = '100%'
        this._renderer.domElement.style.zIndex = '-1'
        this.insertBefore(this._renderer.domElement, this.firstChild)
        this._initStars()
    }

    _initStars() {
        this._geoStars = []
        for (let i = 0; i < Math.min(4000 * this._responsiveNumber, 10000); i++) {
            let star = new three.Vector3(
                Math.random() * 600 - 300,
                Math.random() * 600 - 300,
                Math.random() * 600 - 300
            )

            star.velocity = 0
            star.acceleration = 0.01

            this._geoStars.push(star)
        }

        this._starGeo = new three.BufferGeometry().setFromPoints(this._geoStars)
        const sprite = new three.TextureLoader().load("/assets/images/circle.png")
        const starMaterial = new three.PointsMaterial({
            color: "gray",
            size: 0.3,
            map: sprite
        })

        this._stars = new three.Points(this._starGeo, starMaterial)
        this._scene.add(this._stars)
    }

    cancel() {
        console.log('Canceled sky 3d')
        this._canceled = true
    }

    disconnectedCallback() {
        this.cancel()
    }

    connectedCallback() {
        this.render()
    }

    render() {
        if (this._canceled) {
            this._canceled = false
            return
        }

        const positions = this._starGeo.attributes.position.array

        for (let i = 0; i < this._geoStars.length; i++) {
            const zIndex = i * 3 + 2
            this._geoStars[i].velocity += this._geoStars[i].acceleration
            positions[zIndex] += this._geoStars[i].velocity
            if (positions[zIndex] > 300) {
                positions[zIndex] = -300
                this._geoStars[i].velocity = 0
            }
        }

        this._starGeo.attributes.position.needsUpdate = true
        this._renderer.render(this._scene, this._camera)

        this._stars.rotation.z += this._responsiveNumber * 0.0005
        requestAnimationFrame(this.render.bind(this))
    }
}

customElements.define('star-sky-3d', StarSky3d);
