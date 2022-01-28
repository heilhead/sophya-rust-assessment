use super::components::*;
use crate::asset::*;
use crate::math::*;
use crate::physics::*;
use crate::render::*;
use hecs::*;
use macroquad::prelude::*;

pub struct CharacterSpawnParams {
    pub offset: Vec2,
    pub front_point: Vec2,
    pub texture: TextureResource,
    pub position: Vec2,
    pub physics_body_origin: Vec3,
    pub physics_collider_half_extent: Vec3,
}

// This is a rudimentary scene representation. Since there's not a whole lot going on currently, it
// takes care of everything - from physics to rendering. If/when the code base grows, we'd likely
// split it into smaller pieces. For now though, this should be fine.
pub struct Scene {
    world: World,
    physics: PhysicsWorld,
    scale: f32,
    background_draw_call: Option<SpriteDrawCall>,
    player_character: Option<Entity>,
}

impl Scene {
    pub fn new(scale: f32, background_tex: Option<TextureResource>) -> Self {
        let world = World::new();
        let physics = PhysicsWorld::new();
        let player_character = None;
        let background_draw_call = background_tex.map(|tex| SpriteDrawCall {
            origin: vec2(0.0, 0.0),
            extent: get_texture_size(tex.as_ref()) * scale,
            texture: Some(tex.clone()),
            ..Default::default()
        });

        Self {
            world,
            physics,
            scale,
            background_draw_call,
            player_character,
        }
    }

    // Initialize should be called after all the static objects have been spawned (a.k.a. at the end
    // of scene loading) to precompute static object draw calls and finish any pending
    // initialization.
    pub fn initialize(&mut self) {
        // Update draw calls for static objects so we don't have to do it each frame.
        init_static_scene_objects(&mut self.world);
    }

    pub fn spawn_static_object(
        &mut self,
        asset: &Asset,
        position: Vec2,
        orientation: AssetOrientation,
    ) -> anyhow::Result<()> {
        spawn_static_scene_object(
            &mut self.world,
            &mut self.physics,
            asset,
            position,
            self.scale,
            orientation,
        )?;

        Ok(())
    }

    pub fn spawn_player(&mut self, params: CharacterSpawnParams) {
        self.player_character = Some(spawn_character(&mut self.world, &mut self.physics, self.scale, params));
    }

    // Update player movement speed whenever it changes, or simply each frame.
    pub fn set_player_movement_input(&mut self, input: Vec2, move_speed: f32) {
        if let Some(player_character) = self.player_character {
            // Update character desired velocity, based on input.
            let (vel_comp, phys_body_comp) = self
                .world
                .query_one_mut::<(&mut CharacterVelocityComponent, &PhysicsBodyComponent)>(player_character)
                .unwrap();

            // Store velocity so we can later use it to determine sprite facing direction.
            vel_comp.velocity = screen_to_world(input) * move_speed;

            // Set physics body linear velocity.
            self.physics
                .set_body_linear_velocity_2d(phys_body_comp.handle, vel_comp.velocity);
        }
    }

    // Runs the full scene update. Takes care of character movement, physics and rendering.
    pub fn update(&mut self, dt: f32) {
        // Run physics simulation.
        self.physics.update(dt);
        self.update_dynamic_objects();
        self.render();
    }

    fn update_dynamic_objects(&mut self) {
        // Update characters' transforms and draw calls. There's only the player's character
        // right now, but let's pretend we have a bunch of dynamic characters.
        for (_, (root_transform, sprite_transform, vel_comp, draw_call_comp, phys_body_comp)) in
            self.world.query_mut::<(
                &mut RootTransformComponent,
                &CharacterTransformComponent,
                &CharacterVelocityComponent,
                &mut SpriteDrawCallComponent,
                &PhysicsBodyComponent,
            )>()
        {
            // Update character position based on physics simulation results.
            root_transform.position = self.physics.get_body_translation_2d(phys_body_comp.handle);

            let screen_movement_dir = world_to_screen(vel_comp.velocity);

            // Flip character sprite according to screen-space movement direction.
            if screen_movement_dir.x != 0.0 {
                draw_call_comp.draw_call.flip_x = if screen_movement_dir.x > 0.0 { true } else { false }
            }

            update_sprite_draw_call(
                &mut draw_call_comp.draw_call,
                root_transform.position,
                root_transform.scale,
                sprite_transform.offset,
                sprite_transform.front_point,
            );
        }
    }

    fn render(&mut self) {
        clear_background(BLACK);

        // Render scene background.
        if let Some(background_draw_call) = &self.background_draw_call {
            background_draw_call.draw();
        }

        // Render shadows.
        render_background_geometry(&mut self.world);

        // Render main object sprites.
        render_foreground_geometry(&mut self.world);
    }
}

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
    let origin = world_to_screen(root_position) - extent * 0.5 - offset;
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
    for (_, (root_transform, shadow)) in world.query_mut::<(&RootTransformComponent, &mut SceneObjectShadowComponent)>()
    {
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

                    let collider_extent = vec3(
                        primitive_half_extent_2d.x,
                        primitive_half_extent_2d.y,
                        primitive_half_height,
                    );

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

pub fn spawn_character(
    world: &mut World,
    physics: &mut PhysicsWorld,
    scale: f32,
    params: CharacterSpawnParams,
) -> Entity {
    let mut builder = EntityBuilder::new();

    builder.add(RootTransformComponent {
        position: params.position,
        scale,
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
    let physics_body_origin = params.physics_body_origin * scale + vec3(params.position.x, params.position.y, 0.0);
    let physics_collider_half_extent = params.physics_collider_half_extent * scale;

    builder.add(PhysicsBodyComponent {
        // Ideally, character physics body should be a capsule to be able to walk up the stairs etc.
        handle: physics.create_body_cuboid(
            RigidBodyType::Dynamic,
            physics_body_origin,
            physics_collider_half_extent,
        ),
    });

    world.spawn(builder.build())
}
