use macroquad::prelude::*;
use std::{ops::Deref, sync::Arc};

pub type TextureResource = Arc<TextureWrapper>;

#[derive(PartialEq)]
pub struct TextureWrapper {
    inner: Texture2D,
}

impl TextureWrapper {
    pub(self) fn new(buf: bytes::Bytes) -> Self {
        Self {
            // This assumes that all images are in `png` format. In real world we'd either need some
            // sanity checks around this, or detect image format.
            inner: Texture2D::from_file_with_format(&buf, Some(image::ImageFormat::Png)),
        }
    }
}

impl Drop for TextureWrapper {
    fn drop(&mut self) {
        self.inner.delete();
    }
}

impl Deref for TextureWrapper {
    type Target = Texture2D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn create_texture_resource(buf: bytes::Bytes) -> TextureResource {
    Arc::new(TextureWrapper::new(buf))
}
