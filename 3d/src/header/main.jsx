import React, {useEffect, useState} from 'react'
import {useRef, Suspense} from 'react'
import {Canvas, useFrame, useThree, useLoader} from '@react-three/fiber'
import {PivotControls, OrbitControls, Stage, Gltf, useGLTF, Environment} from '@react-three/drei'
import {ViewPortPosition} from '../utils'

import * as THREE from 'three'
import {Hamburger} from "../models/index.js";

export const R3FHeaderComponent = () => {
    const three = useThree()
    const hamburger = useRef()
    const beerModel = useGLTF('/assets-3d/models/beer-mug.glb')
    const map = useLoader(THREE.TextureLoader, '/assets-3d/models/beermug/baked.jpg')
    map.flipY = false
    console.log(map)
    const material = new THREE.MeshBasicMaterial({
        map,
        transparent: true
    })

    beerModel.scene.traverse((c) => {
        console.log('tiendang ne')
        c.material = material
    })

    console.log(beerModel)
    const [viewport, updateViewport] = useState(three.viewport)

    useFrame((frame) => {
        const elapsedTime = frame.clock.elapsedTime
        if (hamburger.current) {
            hamburger.current.rotation.y = elapsedTime * 0.1
        }
    })

    React.useEffect(() => {
        if (!three) return
    }, [three])

    return <>
        <OrbitControls/>
           {/*<Hamburger ref={hamburger} viewportPosition={[1, 2]} scale={0.5} rotation-z={Math.PI * 0.3}/>*/}
           <primitive object={beerModel.scene} scale={6} transparent={true}/>
    </>
}