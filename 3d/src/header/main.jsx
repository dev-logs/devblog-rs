import React, { useState } from "react";
import { useRef } from "react";
import { useFrame, useThree } from "@react-three/fiber";
import { folder, useControls } from 'leva';
import * as THREE from "three";
import beerFoamVertexShader from '../glsl/beermug/foam/vertex.glsl';
import beerFoamFragmentShader from '../glsl/beermug/foam/fragment.glsl';
import beerVertexShader from '../glsl/beermug/beer/vertex.glsl'
import beerFragmentShader from '../glsl/beermug/beer/fragment.glsl'
import {
  useGLTF,
  Environment,
  OrbitControls
} from "@react-three/drei"
import { useEffect } from "react";

const defaultValueDecimalControl = ({value = 0, step = 0.01, min = 0.1, max = 1}) => {
  return {value, min, max, step}
}

export const R3FHeaderComponent = () => {
  const three = useThree()
  const hamburger = useRef()
  const controls = useControls({
    beerGlass: folder({
      glassColor: '#ffffff',
      glassTransparent: defaultValueDecimalControl({value: 1}),
      glassMetalness: defaultValueDecimalControl({value: 0.6}),
      glassRoughness: defaultValueDecimalControl({value: 0}),
      glassEmissive: '#aa7a13',
    }),
    beerFoam: folder({
      foamWaveSpeed: defaultValueDecimalControl({value: 0.1, step: 0.01, min: 0}),
      foamElevation: defaultValueDecimalControl({value: 0.01, step: 0.01, max: 0.2, min: 0.00}),
      foamColor: '#ffffff',
      foamWaveFrequency: {
        x: 0.1,
        y: 0.2
      },
    }),
    beerLiquid: folder({
      beerColor: '#ff7b00'
    })
  })

  const beerModel = useGLTF("/assets-3d/models/beermug/geometries.glb")

  const glassMaterial = new THREE.MeshPhysicalMaterial({
    color: controls.glassColor,
    roughness: controls.glassRoughness,
    metalness: controls.glassMetalness,
    transmission: controls.glassTransparent,
    emissive: controls.glassEmissive
  })

  const beerMaterial = new THREE.ShaderMaterial({
    vertexShader: beerVertexShader,
    fragmentShader: beerFragmentShader,
    uniforms: {
      uColor: {value: new THREE.Color(controls.beerColor)},
      uTime: {value: 0.0}
    }
  })

  const foamMaterial = new THREE.ShaderMaterial({
    vertexShader: beerFoamVertexShader,
    fragmentShader: beerFoamFragmentShader,
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
    beerMaterial.uniforms.uTime.value = clock.elapsedTime
  })

  beerModel.scene.traverse((c) => {
    switch(c.name) {
      case 'GLASS_BEER':
        c.material = glassMaterial
        break
      case 'BEER':
        c.material = beerMaterial
        const count = c.geometry.attributes.position.count
        const aRandomAttribute = new Float32Array(count)
        for (let i = 0; i < count; i++) {
            aRandomAttribute[i] = Math.random();
        }

        c.geometry.setAttribute('aRandom', new THREE.BufferAttribute(aRandomAttribute, 1))

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
