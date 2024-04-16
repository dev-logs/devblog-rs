import ReactDOM from 'react-dom'
import React, {useRef, useState} from 'react'
import {Canvas, useFrame, extend, useThree} from '@react-three/fiber'
import htm from 'htm'
import {OrbitControls} from 'three/examples/jsm/controls/OrbitControls.js'
import * as THREE from "three"

const html = htm.bind(React.createElement)
extend({OrbitControls})
/**
 * The simple client side rendering to perform
 * react three fiber.
 * Currently, this is the only way to use React Three Fiber on Leptos
 */
const TestMesh = () => {
    const sphereRef = useRef()
    const boxRef = useRef()
    const groupRef = useRef()
    useFrame(({clock}) => {
        if (!boxRef.current || !groupRef.current) return
        const elapsed = clock.elapsedTime
        boxRef.current.rotation.y = elapsed * 0.5
        groupRef.current.rotation.y = elapsed * 0.1
    })

    return html`
        <group ref=${groupRef}>
            <mesh ref=${boxRef} position-x=${-2} scale="1">
                <boxGeometry/>
                <meshBasicMaterial color="orange"/>
            </mesh>
            <mesh ref=${sphereRef} position-x=${2} scale="1">
                <sphereGeometry/>
                <meshBasicMaterial color="mediumpurple"/>
            </mesh>
            <mesh position-y=${-1} rotation-x=${Math.PI * -0.4}>
                <planeGeometry args=${[6, 4]}/>
                    <meshBasicMaterial color="greenyellow" side=${THREE.DoubleSide}/>
            </mesh>
            </groupi>
    `
}

const MainComponent = () => {
    const {camera, gl} = useThree()

    return html`
        <AxesHelper args=${[10]}/>
        <orbitControls args=${[camera, gl.domElement]}/>
        <${TestMesh}/>
    `
}

class ReactThreeFiber extends HTMLElement {
    constructor() {
        super()
    }

    attributeChangedCallback(name, oldValue, newValue) {
        this[name] = newValue
    }

    disconnectedCallback() {
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
