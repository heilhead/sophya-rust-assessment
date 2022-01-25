use macroquad::prelude::*;

#[macroquad::main("sophya-rust-challenge")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        next_frame().await;
    }
}
