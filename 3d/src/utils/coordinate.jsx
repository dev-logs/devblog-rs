import * as THREE from 'three'
import React from 'react'
import {useThree} from '@react-three/fiber'

export const ViewPortPosition = (props) => {
    let {children = []} = props || {}
    const three = useThree()
    const viewport = three.viewport

    console.log('tiendang-debug', children)
    if (!children.length) {
        children = [children]
    }

    const childrenPos = children
        .filter((c) => !!c.props.viewportPosition)
        .map((c) => c.props.viewportPosition)
    const childrenRef = children
        .filter((c) => !!c.props.viewportPosition)
        .map((c) => {
            c.ref = c.ref || React.useRef()
            return c.ref
        })

    console.log(childrenPos)
    React.useEffect(() => {
        for (let i = 0; i < childrenRef.length; i++) {
            const childRef = childrenRef[i]
            const [left, top, right, bottom] = childrenPos[i]
            const child = childRef.current
            if (!child || !child.position) continue

            const bb = new THREE.Box3()
            bb.setFromObject(child)
            let size = bb.getSize(new THREE.Vector3())
            const height = size.y
            const width = size.x

            if (!isNaN(left)) child.position.x = viewport.width / -2 + left + width / 2
            if (!isNaN(right)) child.position.x = viewport.width / 2 - right - width / 2
            if (!isNaN(top)) child.position.y = viewport.height / 2 - top - height / 2
            if (!isNaN(bottom)) child.position.y = viewport.height / -2 + bottom + height / 2
        }
    }, [children, children.length])

    return <>
        <group>
            {children}
        </group>        
    </>
}
