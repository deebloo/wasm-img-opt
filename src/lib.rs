use image;
use image::io::Reader;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ImageOptimizer {}

#[wasm_bindgen]
impl ImageOptimizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn blur(&self, data: &[u8], value: f32) -> Vec<u8> {
        self.read(&data, |img, format, bytes| {
            img.blur(value).write_to(bytes, format).unwrap();
        })
        .unwrap()
    }

    pub fn brighten(&self, data: &[u8], value: i32) -> Vec<u8> {
        self.read(&data, |img, format, bytes| {
            img.brighten(value).write_to(bytes, format).unwrap();
        })
        .unwrap()
    }

    pub fn grayscale(&self, data: &[u8]) -> Vec<u8> {
        self.read(&data, |img, format, bytes| {
            img.grayscale().write_to(bytes, format).unwrap();
        })
        .unwrap()
    }

    pub fn invert(&self, data: &[u8]) -> Vec<u8> {
        self.read(&data, |img, format, bytes| {
            img.invert();

            img.write_to(bytes, format).unwrap();
        })
        .unwrap()
    }

    pub fn thumbnail(&self, data: &[u8], width: u32, height: u32) -> Vec<u8> {
        self.read(&data, |img, format, bytes| {
            img.thumbnail(width, height)
                .write_to(bytes, format)
                .unwrap();
        })
        .unwrap()
    }

    fn read<F>(&self, data: &[u8], f: F) -> Option<Vec<u8>>
    where
        F: FnOnce(&mut image::DynamicImage, image::ImageFormat, &mut Vec<u8>),
    {
        if let Ok(reader) = Reader::new(Cursor::new(data)).with_guessed_format() {
            let format = reader.format().unwrap();

            if let Ok(mut img) = reader.decode() {
                let mut bytes: Vec<u8> = Vec::new();

                f(&mut img, format, &mut bytes);

                Some(bytes)
            } else {
                None
            }
        } else {
            None
        }
    }
}
