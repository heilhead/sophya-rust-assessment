use macroquad::prelude::*;
use once_cell::sync::Lazy;

static WORLD_TO_SCREEN_MATRIX: Lazy<Mat2> = Lazy::new(|| {
    let x_angle = 360.0 - 26.565_f32;
    let y_angle = 180.0 + 26.565_f32;

    let x_rad = x_angle.to_radians();
    let y_rad = y_angle.to_radians();

    let (x_sin, x_cos) = x_rad.sin_cos();
    let (y_sin, y_cos) = y_rad.sin_cos();

    Mat2::from_cols(Vec2::new(x_cos, x_sin), Vec2::new(y_cos, y_sin))
});

static SCREEN_TO_WORLD_MATRIX: Lazy<Mat2> = Lazy::new(|| WORLD_TO_SCREEN_MATRIX.inverse());

#[must_use]
pub fn screen_to_world(input: Vec2) -> Vec2 {
    *SCREEN_TO_WORLD_MATRIX * input
}

#[must_use]
pub fn world_to_screen(input: Vec2) -> Vec2 {
    *WORLD_TO_SCREEN_MATRIX * input
}
