use macroquad::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

// Use `Point` instead of `glam::Vec2`, since glam's serde implementation serializes into a tuple.
#[derive(Deserialize, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[inline]
    pub fn vec2(&self) -> Vec2 {
        vec2(self.x, self.y)
    }
}

impl From<Point> for Vec2 {
    fn from(pt: Point) -> Self {
        pt.vec2()
    }
}

#[derive(Deserialize, Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum AssetOrientation {
    #[serde(rename = "N")]
    North,
    #[serde(rename = "S")]
    South,
    #[serde(rename = "E")]
    East,
    #[serde(rename = "W")]
    West,
}

#[derive(Deserialize)]
pub struct AssetMetadata {
    pub name: String,
    pub orientations: HashMap<AssetOrientation, AssetOrientationData>,
}

impl AssetMetadata {
    pub fn get_orientation(&self, orientation: AssetOrientation) -> anyhow::Result<&AssetOrientationData> {
        if let Some(orientation_data) = self.orientations.get(&orientation) {
            Ok(orientation_data)
        } else {
            anyhow::bail!("asset \"{}\" does not support orientation {orientation:?}", self.name)
        }
    }
}

#[derive(Deserialize)]
pub struct AssetOrientationData {
    pub images: HashMap<String, AssetImageData>,
}

#[derive(Deserialize)]
pub struct AssetImageData {
    #[serde(rename = "type")]
    pub ty: AssetImageType,
    pub url: String,
    pub transform: AssetTransform,
    pub primitives: HashMap<String, AssetPrimitive>,
}

impl AssetImageData {
    pub fn get_primitives(&self, ty: AssetPrimitiveType) -> Option<Vec<AssetPrimitive>> {
        let mut result = vec![];

        for (_, x) in &self.primitives {
            if x.ty == ty {
                result.push(*x);
            }
        }

        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssetImageType {
    Image,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AssetTransform {
    pub position: Point,
    pub scale: Point,
    pub front_point: Point,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AssetPrimitiveType {
    Collider,
    Shadow,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AssetPrimitive {
    #[serde(rename = "type")]
    pub ty: AssetPrimitiveType,
    pub shape: AssetPrimitiveShape,

    // Ideally, these should not be optional and instead shadows and colliders should deserialize
    // into different structs, but that may require a custom deserializer and we don't care about
    // that now.
    pub shadow_type: Option<AssetShapeType>,
    pub height: Option<f32>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AssetShapeType {
    Rect,
}

#[derive(Deserialize, Clone, Copy)]
pub struct AssetPrimitiveShape {
    pub position: Point,
    pub scale: Point,
    pub shape: AssetShapeType,
}
