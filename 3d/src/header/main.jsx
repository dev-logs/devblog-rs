import React, {useEffect, useState} from "react"
import {useRef, Suspense} from "react"
import {Canvas, useFrame, useThree, useLoader} from "@react-three/fiber";
import {
  OrbitControls,
  Stage,
  Gltf,
  useGLTF,
  Environment,
} from "@react-three/drei"
import {ViewPortPosition} from "../utils"

import {Leva, useControls} from 'leva'
import {Perf} from 'r3f-perf'
import * as THREE from "three"
import {Hamburger} from "../models/index.js"
import beermugVertexShader from '../glsl/beermug/vertex.glsl'
import beermugFragmentShader from '../glsl/beermug/fragment.glsl'

const defaultValueDecimalControl = ({value = 0, step = 0.01, min = 0.1, max = 1}) => {
  return {value, min, max, step}
}

export const R3FHeaderComponent = () => {
  const three = useThree()
  const hamburger = useRef()
  const controls = useControls({
    perfVisible: true,
    glassColor: '#ffffff',
    glassTransparent: defaultValueDecimalControl({value: 1}),
    glassMetalness: defaultValueDecimalControl({value: 0.6}),
    glassRoughness: defaultValueDecimalControl({value: 0}),
    glassEmissive: '#aa7a13',
    foamWaveSpeed: defaultValueDecimalControl({value: 0.1, step: 0.01, min: 0}),
    foamElevation: defaultValueDecimalControl({value: 0.1, step: 0.01, max: 0.2, min: 0.00}),
    foamColor: '#ffffff',
    foamWaveFrequency: {
      x: 0.1,
      y: 0.2
    }
  })

  const beerModel = useGLTF("/assets-3d/models/beermug/geometries.glb")
  const map = useLoader(
    THREE.TextureLoader,
    "/assets-3d/models/beermug/baked.jpg",
  )

  map.colorSpace = THREE.SRGBColorSpace
  map.flipY = false

  const glassMaterial = new THREE.MeshPhysicalMaterial({
    color: controls.glassColor,
    roughness: controls.glassRoughness,
    metalness: controls.glassMetalness,
    transmission: controls.glassTransparent,
    emissive: controls.glassEmissive
  })

  const beerMaterial = new THREE.MeshPhongMaterial({
    color: 0xff7b00,
    emissive: 0xff7b00
  })

  const foamMaterial = new THREE.ShaderMaterial({
    vertexShader: beermugVertexShader,
    fragmentShader: beermugFragmentShader,
    uniforms: {
      uRotation: {value: new THREE.Vector3(0, 0, 0)},
      uTime: {value: 1.0},
      uBigWavesSpeed: {value: controls.foamWaveSpeed},
      uBigWavesElevation: {value: controls.foamElevation},
      uBigWavesFrequency: {value: controls.foamWaveFrequency},
      uColor: {value: new THREE.Color(controls.foamColor)}
    }
  })

  useFrame(({clock}) => {
    const delta = clock.getDelta()
    foamMaterial.uniforms.uRotation.value = three.camera.rotation
    foamMaterial.uniforms.uTime.value = clock.elapsedTime
  })

  beerModel.scene.traverse((c) => {
    switch(c.name) {
      case 'GLASS_BEER':
        c.material = glassMaterial
        break
      case 'BEER':
        c.material = beerMaterial
        break
      case 'FOAM':
        c.material = foamMaterial
        break
    }
    if (c.name === 'GLASS_BEER') {
      c.material = glassMaterial
    }
  })

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

  return (
    <>
      <Environment
        environmentIntensity={7}
        files={"/assets-3d/studio-light.jpg"}
      />
      <OrbitControls />
      <directionalLight position={beerModel.scene.children[0].position} intensity={0.5} args={[0xff7b00]}/>
      <primitive object={beerModel.scene} scale={6} transparent={true} />
    </>
  )
}
