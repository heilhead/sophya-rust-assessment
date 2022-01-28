mod asset;
mod demo;
mod math;
mod physics;
mod render;
mod scene;

use demo::DemoScene;
use macroquad::prelude::*;

#[macroquad::main("sophya-rust-challenge")]
async fn main() -> anyhow::Result<()> {
    println!("creating demo scene...");

    let mut demo = DemoScene::new(0.25)?;

    println!("populating world...");

    // Set up the demo scene. This would be loaded from some kind of scene file. For the purpose
    // of this demo though we spawn hardcoded objects.
    demo.populate_scene()?;

    println!("entering game loop...");

    loop {
        let dt = macroquad::time::get_frame_time();
        demo.update(dt);
        demo.render();
        next_frame().await;
    }
}
