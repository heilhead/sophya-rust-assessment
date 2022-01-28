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

    let mut demo = DemoScene::new()?;

    println!("populating world...");

    demo.populate_scene()?;

    println!("entering game loop...");

    loop {
        let dt = macroquad::time::get_frame_time();
        demo.update(dt);
        next_frame().await;
    }
}
