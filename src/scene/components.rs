use crate::{asset::*, render::*};
use hecs::Bundle;
use macroquad::prelude::*;
use rapier3d::prelude::RigidBodyHandle;

#[derive(Bundle)]
pub struct SpriteDrawCallComponent {
    pub draw_call: SpriteDrawCall,
}

#[derive(Bundle, Debug)]
pub struct RootTransformComponent {
    pub position: Vec2,
    pub scale: f32,
}

#[derive(Bundle)]
pub struct AssetTransformComponent {
    pub transform: AssetTransform,
}

#[derive(Bundle)]
pub struct PhysicsBodyCollectionComponent {
    pub handles: Vec<RigidBodyHandle>,
}

#[derive(Bundle)]
pub struct CharacterVelocityComponent {
    pub velocity: Vec2,
}

#[derive(Bundle)]
pub struct CharacterTransformComponent {
    pub offset: Vec2,
    pub front_point: Vec2,
}

#[derive(Bundle)]
pub struct PhysicsBodyComponent {
    pub handle: RigidBodyHandle,
}

#[derive(Bundle)]
pub struct SceneObjectShadowComponent {
    pub primitives: Vec<AssetPrimitive>,
    pub draw_calls: Vec<WorldSpaceRectDrawCall>,
}

impl SceneObjectShadowComponent {
    // Update draw calls whenever object transform is updated.
    pub fn update_draw_calls(&mut self, object_position: Vec2, scale: f32) {
        self.draw_calls.clear();

        for primitive in &self.primitives {
            let extent: Vec2 = primitive.shape.scale.vec2() * scale;
            let offset: Vec2 = primitive.shape.position.vec2() * scale;
            let origin = object_position + offset;
            let color = Color::new(0.0, 0.0, 0.0, 0.15);

            self.draw_calls.push(WorldSpaceRectDrawCall { origin, extent, color });
        }
    }
}
