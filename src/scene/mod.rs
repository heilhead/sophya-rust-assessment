mod components;

use crate::{asset::*, math::*, physics::*, render::*};
pub use components::*;
use hecs::*;
use macroquad::prelude::*;
use rapier3d::prelude::RigidBodyType;

pub fn update_sprite_draw_call(
    draw_call: &mut SpriteDrawCall,
    root_position: Vec2,
    scale: f32,
    offset: Vec2,
    front_point: Vec2,
) {
    let tex = draw_call.texture.as_ref().unwrap();
    let extent = get_texture_size(&tex) * scale;
    let offset = offset * scale;

    let mut origin = world_to_screen(root_position);
    origin *= vec2(1.0, -1.0);
    origin -= extent * 0.5;
    origin -= offset;

    let front_point = root_position + front_point * scale;

    draw_call.origin = origin;
    draw_call.extent = extent;
    draw_call.order = front_point.x + front_point.y;
}

pub fn init_static_scene_objects(world: &mut World) {
    // This is a one-off update of all static objects on the scene after it's been loaded.
    // The purpose is to create draw calls and colliders for all geometry.

    // Update sprites.
    for (_, (root_transform, asset, dc)) in world.query_mut::<(
        &RootTransformComponent,
        &AssetTransformComponent,
        &mut SpriteDrawCallComponent,
    )>() {
        update_sprite_draw_call(
            &mut dc.draw_call,
            root_transform.position,
            root_transform.scale,
            asset.transform.position.vec2(),
            asset.transform.front_point.vec2(),
        );
    }

    // Update shadows.
    for (_, (root_transform, shadow)) in world.query_mut::<(&RootTransformComponent, &mut SceneObjectShadowComponent)>() {
        shadow.update_draw_calls(root_transform.position, root_transform.scale);
    }
}

pub fn spawn_static_scene_object(
    world: &mut World,
    physics: &mut PhysicsWorld,
    asset: &Asset,
    position: Vec2,
    scale: f32,
    orientation: AssetOrientation,
) -> anyhow::Result<()> {
    let orientation_data = asset.metadata.get_orientation(orientation)?;

    // The necessary disclaimer here: this is my actual first time working with an ECS, so I'm not
    // aware of the best practices when it comes to splitting entities into components.

    // The task is not clear on how these entities will be used, so I'm assuming a spawn-and-forget
    // type of static scene objects. If we needed to dynamically manage the objects (e.g. constantly
    // spawn and remove them), I'd probably organize components differently so that there's a single
    // entity containing all of the sprites and primitive subcomponents.

    for (name, data) in &orientation_data.images {
        let mut builder = EntityBuilder::new();

        builder.add(RootTransformComponent { position, scale });

        builder.add(AssetTransformComponent {
            transform: data.transform,
        });

        builder.add(SpriteDrawCallComponent {
            draw_call: SpriteDrawCall {
                texture: Some(asset.get_texture_resource(name)?),
                ..Default::default()
            },
        });

        if let Some(primitives) = data.get_primitives(AssetPrimitiveType::Shadow) {
            builder.add(SceneObjectShadowComponent {
                primitives,
                draw_calls: vec![],
            });
        }

        if let Some(primitives) = data.get_primitives(AssetPrimitiveType::Collider) {
            let bodies = primitives
                .iter()
                .map(|primitive| {
                    let primitive_offset = primitive.shape.position.vec2() * scale;
                    let primitive_origin = position + primitive_offset;
                    let primitive_half_height = primitive.height.unwrap() * scale * 0.5;
                    let primitive_half_extent_2d = primitive.shape.scale.vec2() * scale * 0.5;

                    // Body position should be at the center of the cuboid collider, so add half-extent
                    // to the primitive origin.
                    let body_origin = vec3(
                        primitive_origin.x + primitive_half_extent_2d.x,
                        primitive_origin.y + primitive_half_extent_2d.y,
                        primitive_half_height,
                    );

                    let collider_extent = vec3(primitive_half_extent_2d.x, primitive_half_extent_2d.y, primitive_half_height);

                    // Again, the assumption here is that scene objects are immovable and have fully
                    // static physics bodies.
                    physics.create_body_cuboid(RigidBodyType::Static, body_origin, collider_extent)
                })
                .collect();

            builder.add(PhysicsBodyCollectionComponent { handles: bodies });
        }

        world.spawn(builder.build());
    }

    Ok(())
}

pub struct CharacterSpawnParams {
    pub offset: Vec2,
    pub front_point: Vec2,
    pub texture: TextureResource,
    pub position: Vec2,
    pub scale: f32,
    pub physics_body_origin: Vec3,
    pub physics_collider_half_extent: Vec3,
}

pub fn spawn_character(world: &mut World, physics: &mut PhysicsWorld, params: CharacterSpawnParams) -> Entity {
    let mut builder = EntityBuilder::new();

    builder.add(RootTransformComponent {
        position: params.position,
        scale: params.scale,
    });
    builder.add(CharacterVelocityComponent {
        velocity: vec2(0.0, 0.0),
    });
    builder.add(CharacterTransformComponent {
        offset: params.offset,
        front_point: params.front_point,
    });
    builder.add(SpriteDrawCallComponent {
        draw_call: SpriteDrawCall {
            texture: Some(params.texture),
            ..Default::default()
        },
    });

    // Origin should be the center of the cuboid.
    let physics_body_origin =
        params.physics_body_origin * params.scale + vec3(params.position.x, params.position.y, 0.0);
    let physics_collider_half_extent = params.physics_collider_half_extent * params.scale;

    builder.add(PhysicsBodyComponent {
        handle: physics.create_body_cuboid(
            RigidBodyType::Dynamic,
            physics_body_origin,
            physics_collider_half_extent,
        ),
    });

    world.spawn(builder.build())
}
