import React from "react"
import { useFrame, useThree } from "@react-three/fiber"
import { folder, useControls } from 'leva'
import * as THREE from "three"
import beerFoamVertexShader from '../glsl/beermug/foam/vertex.glsl'
import beerFoamFragmentShader from '../glsl/beermug/foam/fragment.glsl'
import beerVertexShader from '../glsl/beermug/beer/vertex.glsl'
import beerFragmentShader from '../glsl/beermug/beer/fragment.glsl'
import {
  useGLTF,
  useTexture
} from "@react-three/drei"
import { getControlValue } from "../utils"

export const BeerMugDefaultConfig = {
    beerGlass: folder({
      glassColor: '#ffffff',
      glassTransparent: getControlValue({value: 1}),
      glassMetalness: getControlValue({value: 0.6}),
      glassRoughness: getControlValue({value: 0}),
      glassEmissive: '#aa7a13',
    }),
    beerFoam: folder({
      foamWaveSpeed: getControlValue({value: 0.1, step: 0.01, min: 0}),
      foamElevation: getControlValue({value: 0.01, step: 0.01, max: 0.2, min: 0.00}),
      foamColor: '#ffffff',
      foamWaveFrequency: {
        x: 0.1,
        y: 0.2
      },
    }),
    beerLiquid: folder({
      beerColor: '#ff7b00'
    })
  }

export const BeerMug = (props) => {
  const three = useThree()
  const controls = useControls(BeerMugDefaultConfig)
  const envMap = useTexture('/assets-3d/studio-room.jpg')

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
            aRandomAttribute[i] = Math.random()
        }

        c.geometry.setAttribute('aRandom', new THREE.BufferAttribute(aRandomAttribute, 1))

        break
      case 'FOAM':
        c.material = foamMaterial
        break
    }
  })

  return (
    <>
      {beerModel && <primitive object={beerModel.scene} scale={6} transparent={true} {...props} />}
    </>
  )
}

useGLTF.preload('/assets-3d/models/beermug/geometries.glb')
