mod resource;

use crate::{math::*, scene::*};
use macroquad::prelude::*;
pub use resource::*;
use std::cmp::Ordering;

pub struct WorldSpaceRectDrawCall {
    pub origin: Vec2,
    pub extent: Vec2,
    pub color: Color,
}

impl WorldSpaceRectDrawCall {
    pub fn draw(&self) {
        // Seems to be something wrong with the world-to-screen matrix, so we manually invert Y-axis
        // in the screen space.
        let invert = vec2(1.0, -1.0);

        let v1 = world_to_screen(self.origin) * invert;
        let v2 = world_to_screen(self.origin + vec2(self.extent.x, 0.0)) * invert;
        let v3 = world_to_screen(self.origin + vec2(self.extent.x, self.extent.y)) * invert;
        let v4 = world_to_screen(self.origin + vec2(0.0, self.extent.y)) * invert;

        // Drawing as separate triangles is obviously suboptimal, but in the interests of time we
        // won't be combining quads into a single mesh.
        draw_triangle(v1, v2, v3, self.color);
        draw_triangle(v1, v3, v4, self.color);
    }
}

#[derive(Default)]
pub struct SpriteDrawCall {
    pub origin: Vec2,
    pub extent: Vec2,
    pub texture: Option<TextureResource>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub order: f32,
}

impl SpriteDrawCall {
    pub fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(self.extent),
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            ..Default::default()
        };
        let tex = self.texture.as_ref().unwrap();

        draw_texture_ex(**tex.as_ref(), self.origin.x, self.origin.y, WHITE, params);
    }
}

pub fn render_background_geometry(world: &mut hecs::World) {
    // This renders shadows without sorting, since they'll always be behind other objects.
    for (_, shadow) in world.query_mut::<&mut SceneObjectShadowComponent>() {
        // Dispatch draw calls.
        for draw_call in &shadow.draw_calls {
            draw_call.draw();
        }
    }
}

pub fn render_foreground_geometry(world: &mut hecs::World) {
    // Ideally, we'd not recreate the vector here to avoid allocations in render loop, but who cares.
    let mut draw_calls = vec![];

    // Collect draw calls.
    for (_, dc) in world.query_mut::<&SpriteDrawCallComponent>() {
        draw_calls.push(&dc.draw_call);
    }

    // Here we'd cull, pigeonhole sort and batch draw calls.
    draw_calls.sort_by(|a, b| a.order.partial_cmp(&b.order).unwrap_or(Ordering::Equal));

    // Dispatch draw calls.
    for draw_call in draw_calls {
        draw_call.draw();
    }
}

#[inline]
pub fn get_texture_size(tex: &Texture2D) -> Vec2 {
    vec2(tex.width(), tex.height())
}