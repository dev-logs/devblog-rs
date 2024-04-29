import { useGLTF, useTexture } from "@react-three/drei"
import * as THREE from 'three'

export const MacOne = (props) => {
  const model = useGLTF('/assets-3d/models/macone/geometries.glb')
  if (model) {
    const texture = useTexture('/assets-3d/models/macone/texture-macone-mouse.jpg')
    texture.colorSpace = THREE.SRGBColorSpace
    texture.flipY = false

    model.scene.traverse((c) => {
      c.material = new THREE.MeshBasicMaterial({
        map: texture
      })
    })
  }

  return <>
    <primitive scale={10} object={model.scene}/>
  </>
}

useGLTF.preload('/assets-3d/models/macone/geometries.glb')
