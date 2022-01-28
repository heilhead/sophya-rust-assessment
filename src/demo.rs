use crate::{asset::*, scene::*};
use macroquad::prelude::*;

const PLAYER_MOVEMENT_SPEED: f32 = 200.0;
const DEMO_SCENE_SCALE: f32 = 0.25;

pub fn get_movement_input() -> Vec2 {
    let mut movement_dir = vec2(0.0, 0.0);

    if is_key_down(KeyCode::D) {
        movement_dir.x += 1.0;
    }

    if is_key_down(KeyCode::A) {
        movement_dir.x -= 1.0;
    }

    if is_key_down(KeyCode::S) {
        movement_dir.y += 1.0;
    }

    if is_key_down(KeyCode::W) {
        movement_dir.y -= 1.0;
    }

    movement_dir.normalize_or_zero()
}

pub struct DemoScene {
    scene: Scene,
}

impl DemoScene {
    pub fn new() -> anyhow::Result<Self> {
        // This is here since we don't have asset metadata available for the character.
        let player_spawn_params = CharacterSpawnParams {
            offset: vec2(0.0, 160.0),
            front_point: vec2(-150.0, -150.0),
            texture: load_texture_from_file("assets/character.png")?,
            position: vec2(500.0, 200.0),
            physics_body_origin: vec3(40.0, 40.0, 160.0),
            physics_collider_half_extent: vec3(40.0, 40.0, 160.0),
        };

        let mut scene = Scene::new(DEMO_SCENE_SCALE, Some(load_texture_from_file("assets/map.png")?));
        scene.spawn_player(player_spawn_params);

        Ok(Self { scene })
    }

    pub fn populate_scene(&mut self) -> anyhow::Result<()> {
        // Set up the demo scene. This would be loaded from some kind of scene file. For the purpose
        // of this demo though we spawn hardcoded objects.

        let assets = ["assets/chair.json", "assets/table.json", "assets/bookshelf.json"];
        let assets = load_asset_bundle(&assets)?;

        // Chairs
        self.scene
            .spawn_static_object(&assets[0], vec2(400.0, 20.0), AssetOrientation::East)?;
        self.scene
            .spawn_static_object(&assets[0], vec2(500.0, 20.0), AssetOrientation::West)?;
        self.scene
            .spawn_static_object(&assets[0], vec2(400.0, 120.0), AssetOrientation::East)?;
        self.scene
            .spawn_static_object(&assets[0], vec2(500.0, 120.0), AssetOrientation::West)?;

        // Tables
        self.scene
            .spawn_static_object(&assets[1], vec2(440.0, 5.0), AssetOrientation::East)?;
        self.scene
            .spawn_static_object(&assets[1], vec2(440.0, 105.0), AssetOrientation::East)?;

        // Bookshelves
        self.scene
            .spawn_static_object(&assets[2], vec2(350.0, -30.0), AssetOrientation::East)?;
        self.scene
            .spawn_static_object(&assets[2], vec2(350.0, 80.0), AssetOrientation::East)?;
        self.scene
            .spawn_static_object(&assets[2], vec2(400.0, -80.0), AssetOrientation::South)?;
        self.scene
            .spawn_static_object(&assets[2], vec2(510.0, -80.0), AssetOrientation::South)?;

        // Finish scene 'loading'.
        self.scene.initialize();

        Ok(())
    }

    pub fn update(&mut self, dt: f32) {
        self.scene
            .set_player_movement_input(get_movement_input(), PLAYER_MOVEMENT_SPEED);
        self.scene.update(dt);
    }
}
