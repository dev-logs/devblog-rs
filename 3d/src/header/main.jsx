import React from "react";
import {
    Environment,
    OrbitControls,
    useTexture
} from "@react-three/drei";
import { BeerMug } from "../models/beermug"
import { MacOne } from "../models/macone"

export const R3FHeaderComponent = () => {
  return (
    <>
      <Environment
        environmentIntensity={3}
        files={"/assets-3d/studio-room.jpg"}
      />
      <OrbitControls />
      <BeerMug/>
      <MacOne/>
    </>
  )
}

useTexture.preload('/assets-3d/studio-room.jpg')
