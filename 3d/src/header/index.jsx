import React from 'react'
import {Canvas} from '@react-three/fiber'
import ReactDOM from 'react-dom'
import {R3FHeaderComponent} from './main.jsx'

export class R3fHeader1 extends HTMLElement {
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
        ReactDOM.createRoot(shadowRoot).render(
            <React.StrictMode>
                <Canvas>
                    <R3FHeaderComponent/>
                </Canvas>
           </React.StrictMode>
        )
    }
}
