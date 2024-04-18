import React from 'react'
import htm from 'htm'
const html = htm.bind(React.createElement)
import { useGLTF, PivotControls } from '@react-three/drei'

export function Hamburger(props) {
    const { nodes, materials } = useGLTF('/assets/models/hamburger.glb')

    return html`
        <group scale=${props.scale} dispose=${null}>
            <${PivotControls} anchor=${[0, 0, 0]} depthTest=${false}>
            <mesh
                castShadow
                receiveShadow
                geometry=${nodes.bottomBun.geometry}
                material=${materials.BunMaterial}
                userData=${{ name: 'bottomBun' }}
            />
            </PivotControls>
            <mesh
                castShadow
                receiveShadow
                geometry=${nodes.meat.geometry}
                material=${materials.SteakMaterial}
                position=${[0, 2.82, 0]}
                userData=${{ name: 'meat' }}
            />
            <mesh
                castShadow
                receiveShadow
                geometry=${nodes.cheese.geometry}
                material=${materials.CheeseMaterial}
                position=${[0, 3.04, 0]}
                userData=${{ name: 'cheese' }}
            />
            <${PivotControls} anchor=${[0, 1, 0]} depthTest=${false}>
            <mesh
                castShadow
                receiveShadow
                geometry=${nodes.topBun.geometry}
                material=${materials.BunMaterial}
                position=${[0, 6.77, 0]}
                rotation-x=${Math.PI * 0.5}
                userData=${{ name: 'topBun' }}
            />
            </PivotControls> 
        </group>
    `
}

useGLTF.preload('/assets/models/hamburger.glb')
