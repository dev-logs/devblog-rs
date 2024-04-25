import React, {useEffect, useState} from "react";
import {useRef, Suspense} from "react";
import {Canvas, useFrame, useThree, useLoader} from "@react-three/fiber";
import {
  OrbitControls,
  Stage,
  Gltf,
  useGLTF,
  Environment,
} from "@react-three/drei";
import {ViewPortPosition} from "../utils"

import * as THREE from "three";
import {Hamburger} from "../models/index.js"

export const R3FHeaderComponent = () => {
  const three = useThree();
  const hamburger = useRef();
  const beerModel = useGLTF("/assets-3d/models/beermug/geometries.glb");
  const map = useLoader(
    THREE.TextureLoader,
    "/assets-3d/models/beermug/baked.jpg",
  );


  // const studioMap = useLoader(
  //   THREE.TextureLoader,
  //   "/assets-3d/studio-light.jpg",
  // )


  map.colorSpace = THREE.SRGBColorSpace;
  map.flipY = false
  let material = new THREE.MeshBasicMaterial({
    map,
    transparent: true,
  });

  // material = new THREE.MeshStandardMaterial({
  //     roughness: 0.1,
  //     transparent: true
  // })

  const glassMaterial = new THREE.MeshPhysicalMaterial({
    color: 0xffffff,
    roughness: 0.0,
    metalness: 0.6,
    transmission: 1, // Add transparency
    emissive: 0xaa7a13,
  })

  const beerMaterial = new THREE.MeshPhongMaterial({
    color: 0xff7b00,
    emissive: 0xff7b00
  })

  const foamMaterial = new THREE.MeshPhysicalMaterial({
   color: 0xe7eff3,
   roughness: 0.0,
   transmission: 0.09, // Add transparency
   emissive: 0xe7eff3,
  })

  beerModel.scene.traverse((c) => {
    switch(c.name) {
      case 'GLASS_BEER':
        c.material = glassMaterial
        break
      case 'BEER':
        c.material = beerMaterial
        console.log('ok')
        break
      case 'FOAM':
        c.material = foamMaterial
        break
    }
    if (c.name === 'GLASS_BEER') {
      c.material = glassMaterial
    }
  })

  const [viewport, updateViewport] = useState(three.viewport);

  useFrame((frame) => {
    const elapsedTime = frame.clock.elapsedTime;
    if (hamburger.current) {
      hamburger.current.rotation.y = elapsedTime * 0.1;
    }
  });

  React.useEffect(() => {
    if (!three) return;
  }, [three]);

  return (
    <>
      <Environment
        environmentIntensity={7}
        files={"/assets-3d/studio-light.jpg"}
      />
      <OrbitControls />
      <directionalLight position={beerModel.scene.children[0].position} intensity={0.5} args={[0xff7b00]}/>
      {/*<Hamburger ref={hamburger} viewportPosition={[1, 2]} scale={0.5} rotation-z={Math.PI * 0.3}/>*/}
      <primitive object={beerModel.scene} scale={6} transparent={true} />
    </>
  );
};
