import React from "react";
import {
    Environment,
    OrbitControls
} from "@react-three/drei";
import { BeerMug } from "../models/beermug";

export const R3FHeaderComponent = () => {
  return (
    <>
      <Environment
        environmentIntensity={7}
        files={"/assets-3d/studio-light.jpg"}
      />
      <OrbitControls />
      {/* <directionalLight position={beerModel.scene.children[0].position} intensity={0.5} args={[0xff7b00]}/> */}
      <BeerMug/>
    </>
  )
}
