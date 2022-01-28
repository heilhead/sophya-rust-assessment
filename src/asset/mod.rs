mod loader;
mod metadata;

use crate::render::TextureResource;
pub use loader::*;
pub use metadata::*;
use std::collections::HashMap;

pub type AssetResourceList = HashMap<String, TextureResource>;

pub struct Asset {
    pub metadata: AssetMetadata,
    pub resources: AssetResourceList,
}

impl Asset {
    pub fn get_texture_resource(&self, name: &String) -> anyhow::Result<TextureResource> {
        if let Some(tex) = self.resources.get(name) {
            Ok(tex.clone())
        } else {
            anyhow::bail!("resource not available: {name}")
        }
    }
}
