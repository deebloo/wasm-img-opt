use image;
use image::io::Reader as ImageReader;
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
        ImageOptimizer {}
    }

    pub fn blur(&self, data: &[u8], value: f32) -> Vec<u8> {
        let reader = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .unwrap();

        let format = reader.format().unwrap();

        let img = reader.decode().unwrap();

        let mut bytes: Vec<u8> = Vec::new();

        img.blur(value).write_to(&mut bytes, format).unwrap();

        bytes
    }

    pub fn brighten(&self, data: &[u8], value: i32) -> Vec<u8> {
        let reader = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .unwrap();

        let format = reader.format().unwrap();

        let img = reader.decode().unwrap();

        let mut bytes: Vec<u8> = Vec::new();

        img.brighten(value).write_to(&mut bytes, format).unwrap();

        bytes
    }

    pub fn grayscale(&self, data: &[u8]) -> Vec<u8> {
        let reader = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .unwrap();

        let format = reader.format().unwrap();

        let img = reader.decode().unwrap();

        let mut bytes: Vec<u8> = Vec::new();

        img.grayscale().write_to(&mut bytes, format).unwrap();

        bytes
    }

    pub fn invert(&self, data: &[u8]) -> Vec<u8> {
        let reader = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .unwrap();

        let format = reader.format().unwrap();

        let mut img = reader.decode().unwrap();
        img.invert();

        let mut bytes: Vec<u8> = Vec::new();

        img.write_to(&mut bytes, format).unwrap();

        bytes
    }
}
