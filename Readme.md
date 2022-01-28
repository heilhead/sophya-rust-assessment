# Sophya Rust Challenge

The goal of this challenge is to create an isometric, decorated scene in which the character can move around the objects in the room.

## The scene should:

- be in 2d,
- use all of the assets at least twice, in two different orientations each
  - use the asset details to draw programatic shadows underneath each asset
- have 3d collision, driven by the collider definitions in the assets
  - i.e. the character will stop walking against the objects in the room.
- have a debug menu to enable visualization of the 3d colliders
- use WASD or arrow keys to control the character
  - `W` should go `North`
  - `D` should go `East`
  - etc.

## The code should:

- follow an ECS architectural design, with the `hecs` ecs being the primary state and data store of the scene.
- use `macroquad` for any windowing, input, and rendering.
- use `rapier3d` for 3d physics state management and collision detection
- use `egui` for the immediate mode ui. (with `egui-macroquad` to make things easier)
- be well organized, with distinct sepration of concerns and single responsibilities.
- `safe` (in rust terms)

## Utilities provided:

- matricies and utility functions to convert between `2d` and `isometric` space.

## Safe Assumptions / Tips:

- the center of map image should be used as the origin for the isometric space.
- rapier3d doesn't love large numbers / spaces, you may have to scale data in/out to make it perform well.
- the character won't animate.
- the walls won't have collision.

## Asset Format

Coordinate Definitions:

- `2d space`: +x is right, +y is up.
- `isometric space`: +x is diagonally down and right, +y is diagonally down and left
- `isometric cardinal directions`:

  - `N` is up and right (`-y`)
  - `E` is down and right (`+x`)
  - `S` is down and left (`+y`)
  - `W` is up and left (`-x`)

- asset
  - orientations: there are the possible rotations of the objects, facing either `N`, `E`, `S`, or `W`,
    - images: each orientation has at least 1 image.
      - transform: transform data, in `2d space`, the origin of the image is the _center_
        - position
        - scale
        - frontPoint: this defines the front most point of the object, useful for depth sorting the objects.
        - type: image
        - primitives: these are the sub components of the images, In this case, 3d colliders, and shadows to render
          - `shadow` type
            - shape: positional data in `isometric space` _relative_ to the origin of the image
          - `collider` type
            - shape: also position data in the `isometric space`, has a height value for accurate collider creation

## Result

[Demo video](https://youtu.be/FXlsV4iHziI)
