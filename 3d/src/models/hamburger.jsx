import React, {forwardRef} from 'react'
import { useGLTF } from '@react-three/drei'

export const Hamburger = forwardRef((props, ref) => {
    const { nodes, materials } = useGLTF('/assets-3d/models/hamburger.glb')
    return (
        <group {...props} dispose={null} ref={ref}>
            <group scale={0.3} name="Scene">
                <mesh
                    name="bottomBun"
                    castShadow
                    receiveShadow
                    geometry={nodes.bottomBun.geometry}
                    material={materials.BunMaterial}
                    userData={{ name: 'bottomBun' }}
                />
                <mesh
                    name="meat"
                    castShadow
                    receiveShadow
                    geometry={nodes.meat.geometry}
                    material={materials.SteakMaterial}
                    position={[0, 2.817, 0]}
                    userData={{ name: 'meat' }}
                />
                <mesh
                    name="cheese"
                    castShadow
                    receiveShadow
                    geometry={nodes.cheese.geometry}
                    material={materials.CheeseMaterial}
                    position={[0, 3.04, 0]}
                    userData={{ name: 'cheese' }}
                />
                <mesh
                    name="topBun"
                    castShadow
                    receiveShadow
                    geometry={nodes.topBun.geometry}
                    material={materials.BunMaterial}
                    position={[0, 1.771, 0]}
                    userData={{ name: 'topBun' }}
                />
            </group>
        </group>
    )
})

useGLTF.preload('/assets-3d/models/hamburger.glb')
