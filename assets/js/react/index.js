import ReactDOM from 'react-dom'
import {useRef, Suspense} from 'react'
import * as React from 'react'
import {Canvas, useFrame, useThree, useLoader} from '@react-three/fiber'
import htm from 'htm'
import * as THREE from "three"
import {GLTFLoader} from 'three/examples/jsm/loaders/GLTFLoader.js'
import { DRACOLoader } from 'three/examples/jsm/loaders/DRACOLoader.js'
import {OrbitControls, PivotControls, Html} from '@react-three/drei'
import {Hamburger} from './hamburger.js'

const html = htm.bind(React.createElement)
/**
 * The simple client side rendering to perform
 * react three fiber.
 * Currently, this is the only way to use React Three Fiber on Leptos
 */

const Helmet = () => {
    const model = useLoader(
        GLTFLoader,
        '/assets/models/FlightHelmet/glTF/FlightHelmet.gltf',
        (loader) =>
        {
            const dracoLoader = new DRACOLoader()
            dracoLoader.setDecoderPath('/assets/models/draco/')
            loader.setDRACOLoader(dracoLoader)
        }
    )

    return html`
        <primitive object=${model.scene}/>
    `
}

const TestMesh = React.forwardRef(() => {
    const sphereRef = useRef()
    const boxRef = useRef()
    const groupRef = useRef()
    const translateRef = useRef()

    const three = useThree()
    console.log(three)
   return html`
        <group ref=${groupRef}>
            <directionalLight intensity=${3}/>
            <${OrbitControls} makeDefault enableDamping=${true}/>
            <${PivotControls} anchor=${[0, 0, 0]} depthTest=${false}>
                <mesh position-x=${-2} scale="1">
                    <boxGeometry/>
                    <meshBasicMaterial color="orange"/>
                </mesh>
            </PivotControls>  
            <mesh ref=${sphereRef} position-x=${2} scale="1">
                <sphereGeometry/>
                <meshBasicMaterial color="mediumpurple"/>
            </mesh>
            <mesh position-y=${-1} rotation-x=${Math.PI * -0.4}>
                <planeGeometry args=${[6, 4]}/>
                <meshBasicMaterial color="greenyellow" side=${THREE.DoubleSide}/>
            </mesh>
            <${Suspense}>
                <${Hamburger} scale=${0.3}/>
            </Suspense>    
        </groupi>
    `
})

const MainComponent = () => {
    return html`
        <AxesHelper args=${[10]}/>
        <${Suspense}>
            <${TestMesh}/>
        </Suspense>
    `
}

class ReactThreeFiber extends HTMLElement {
    constructor() {
        super()
    }

    attributeChangedCallback(name, oldValue, newValue) {
        this[name] = newValue
    }

    connectedCallback() {
        this.render()
    }

    render() {
        const shadowRoot = this.attachShadow({mode: 'open'})
        ReactDOM.render(html`
            <${Canvas}
            width="${this.width}"
            height="${this.height}">
                <${MainComponent}/>
            </Canvas>
        `, shadowRoot)
    }
}

customElements.define('react-root', ReactThreeFiber)
