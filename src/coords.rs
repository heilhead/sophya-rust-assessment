use glam::{Mat2, Mat4, Vec2, Vec3, Vec4};
use once_cell::sync::Lazy;

static ISO_TO_D2_MATRIX: Lazy<Mat2> = Lazy::new(|| {
    let x_angle = 360.0 - 26.565_f32;
    let y_angle = 180.0 + 26.565_f32;

    let x_rad = x_angle.to_radians();
    let y_rad = y_angle.to_radians();

    let (x_sin, x_cos) = x_rad.sin_cos();
    let (y_sin, y_cos) = y_rad.sin_cos();

    Mat2::from_cols(Vec2::new(x_cos, x_sin), Vec2::new(y_cos, y_sin))
});

static D2_TO_ISO_MATRIX: Lazy<Mat2> = Lazy::new(|| ISO_TO_D2_MATRIX.inverse());

#[must_use]
pub fn d2_to_iso(input: Vec2) -> Vec2 {
    *D2_TO_ISO_MATRIX * input
}

#[must_use]
pub fn iso_to_d2(input: Vec2) -> Vec2 {
    *ISO_TO_D2_MATRIX * input
}
