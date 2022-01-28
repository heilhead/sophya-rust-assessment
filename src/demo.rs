use crate::{asset::*, math::*, physics::*, render::*, scene::*};
use hecs::{Entity, World};
use macroquad::prelude::*;

const CHARACTER_MOVE_SPEED: f32 = 200.0;

pub fn get_movement_input() -> Vec2 {
    let mut movement_dir = vec2(0.0, 0.0);

    if is_key_down(KeyCode::D) {
        movement_dir.x += 1.0;
    }

    if is_key_down(KeyCode::A) {
        movement_dir.x -= 1.0;
    }

    if is_key_down(KeyCode::S) {
        movement_dir.y -= 1.0;
    }

    if is_key_down(KeyCode::W) {
        movement_dir.y += 1.0;
    }

    movement_dir.normalize_or_zero()
}

// This is the equivalent of a `Scene` object. The scene would hold an ECS world, a physics world
// and a bunch of other management/utility things. It'd also be able to update itself
// (i.e. character movement and physics simulation) and expose a some kind of draw call iteration
// API for the renderer. Sadly, at the start I was not familiar with neither ECS, nor `macroquad`'s
// rendering API so I had to figure out along the way, and in the end I didn't have time to refactor
// and reorganize stuff in a proper way.
//
// So what we have here is this:
//  - the demo scene takes care of the asset loading and game world population;
//  - handles game tick: updates player character movement and performs the physics update;
//  - also knows how to render itself (background, shadows, foreground);
//
// Yeah, not ideal, but here we are. I'd do a lot differently if I was familiar with these APIs from
// the start.
pub struct DemoScene {
    world: World,
    physics: PhysicsWorld,
    scale: f32,
    tex_background: TextureResource,
    player_character: Entity,
}

impl DemoScene {
    pub fn new(scale: f32) -> anyhow::Result<Self> {
        let tex_background = load_texture_from_file("assets/map.png")?;
        let mut world = World::new();
        let mut physics = PhysicsWorld::new();

        let player_spawn_params = CharacterSpawnParams {
            offset: vec2(0.0, 160.0),
            front_point: vec2(-150.0, -150.0),
            texture: load_texture_from_file("assets/character.png")?,
            position: vec2(500.0, 200.0),
            scale,
            physics_body_origin: vec3(40.0, 40.0, 160.0),
            physics_collider_half_extent: vec3(40.0, 40.0, 160.0),
        };

        let player_character = spawn_character(&mut world, &mut physics, player_spawn_params);

        Ok(Self {
            world,
            physics,
            scale,
            tex_background,
            player_character,
        })
    }

    pub fn populate_scene(&mut self) -> anyhow::Result<()> {
        let assets = ["assets/chair.json", "assets/table.json", "assets/bookshelf.json"];
        let assets = load_asset_bundle(&assets)?;
        let scale = self.scale;
        let world = &mut self.world;
        let physics = &mut self.physics;

        // Chairs
        spawn_static_scene_object(
            world,
            physics,
            &assets[0],
            vec2(400.0, 20.0),
            scale,
            AssetOrientation::East,
        )?;
        spawn_static_scene_object(
            world,
            physics,
            &assets[0],
            vec2(500.0, 20.0),
            scale,
            AssetOrientation::West,
        )?;
        spawn_static_scene_object(
            world,
            physics,
            &assets[0],
            vec2(400.0, 120.0),
            scale,
            AssetOrientation::East,
        )?;
        spawn_static_scene_object(
            world,
            physics,
            &assets[0],
            vec2(500.0, 120.0),
            scale,
            AssetOrientation::West,
        )?;

        // Tables
        spawn_static_scene_object(
            world,
            physics,
            &assets[1],
            vec2(440.0, 5.0),
            scale,
            AssetOrientation::East,
        )?;
        spawn_static_scene_object(
            world,
            physics,
            &assets[1],
            vec2(440.0, 105.0),
            scale,
            AssetOrientation::East,
        )?;

        // Bookshelves
        spawn_static_scene_object(
            world,
            physics,
            &assets[2],
            vec2(350.0, -30.0),
            scale,
            AssetOrientation::East,
        )?;
        spawn_static_scene_object(
            world,
            physics,
            &assets[2],
            vec2(350.0, 80.0),
            scale,
            AssetOrientation::East,
        )?;
        spawn_static_scene_object(
            world,
            physics,
            &assets[2],
            vec2(400.0, -80.0),
            scale,
            AssetOrientation::South,
        )?;
        spawn_static_scene_object(
            world,
            physics,
            &assets[2],
            vec2(510.0, -80.0),
            scale,
            AssetOrientation::South,
        )?;

        // Update draw calls for static objects so we don't have to do it each frame.
        init_static_scene_objects(world);

        Ok(())
    }

    pub fn update(&mut self, dt: f32) {
        self.update_player_character_movement();
        self.update_physics(dt);
        self.update_dynamic_objects();
    }

    fn update_player_character_movement(&mut self) {
        // Update character desired velocity, based on input.
        let (vel_comp, phys_body_comp) = self
            .world
            .query_one_mut::<(&mut CharacterVelocityComponent, &PhysicsBodyComponent)>(self.player_character)
            .unwrap();

        // Store velocity so we can later use it to determine sprite facing direction.
        vel_comp.velocity = screen_to_world(get_movement_input()) * CHARACTER_MOVE_SPEED;

        // Set physics body linear velocity.
        self.physics
            .set_body_linear_velocity_2d(phys_body_comp.handle, vel_comp.velocity);
    }

    fn update_physics(&mut self, dt: f32) {
        // Run physics simulation.
        self.physics.step(dt);
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

    pub fn render(&mut self) {
        clear_background(BLACK);

        // Render scene background.
        {
            // Nothing fancy here...
            let draw_size = get_texture_size(&self.tex_background) * self.scale;
            let params = DrawTextureParams {
                dest_size: Some(draw_size),
                ..Default::default()
            };

            draw_texture_ex(**self.tex_background, 0.0, 0.0, WHITE, params);
        }

        // Render shadows.
        render_background_geometry(&mut self.world);

        // Render main object sprites.
        render_foreground_geometry(&mut self.world);
    }
}
