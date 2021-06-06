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
pub struct ImagePtr {
    pub mem_loc: *const u8,
    pub size: usize,
}

impl ImagePtr {
    pub fn new(data: &Vec<u8>) -> Self
        Self {
            mem_loc: data.as_ptr(),
            size: data.len(),
        }
    }
}

#[wasm_bindgen]
pub struct ImageOptimizer {}

#[wasm_bindgen]
impl ImageOptimizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn blur(&self, data: &[u8], value: f32) -> Option<Vec<u8>> {
        self.read_with_format(&data, |img, format, bytes| {
            img.blur(value)
                .write_to(bytes, format)
                .expect("failed to blur image");
        })
    }

    pub fn blur_as_ptr(&self, data: &[u8], value: f32) -> Option<ImagePtr> {
        let image = self.blur(&data, value)?;

        Some(ImagePtr::new(&image))
    }

    pub fn brighten(&self, data: &[u8], value: i32) -> Option<Vec<u8>> {
        self.read_with_format(&data, |img, format, bytes| {
            img.brighten(value)
                .write_to(bytes, format)
                .expect("failed to brighten image");
        })
    }

    pub fn brighten_as_ptr(&self, data: &[u8], value: i32) -> Option<ImagePtr> {
        let image = self.brighten(&data, value)?;

        Some(ImagePtr::new(&image))
    }

    pub fn grayscale(&self, data: &[u8]) -> Option<Vec<u8>> {
        self.read_with_format(&data, |img, format, bytes| {
            img.grayscale()
                .write_to(bytes, format)
                .expect("failed to grayscale image");
        })
    }

    pub fn grayscale_as_ptr(&self, data: &[u8]) -> Option<ImagePtr> {
        let image = self.grayscale(&data)?;

          Some(ImagePtr::new(&image))
    }

    pub fn invert(&self, data: &[u8]) -> Option<Vec<u8>> {
        self.read_with_format(&data, |img, format, bytes| {
            img.invert();

            img.write_to(bytes, format).expect("failed to invert image");
        })
    }

    pub fn invert_as_ptr(&self, data: &[u8]) -> Option<ImagePtr> {
        let image = self.invert(&data)?;

        Some(ImagePtr::new(&image))
    }

    pub fn thumbnail(&self, data: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
        self.read_with_format(&data, |img, format, bytes| {
            img.thumbnail(width, height)
                .write_to(bytes, format)
                .expect("failed to thumbnail image");
        })
    }

    pub fn thumbnail_as_ptr(&self, data: &[u8], width: u32, height: u32) -> Option<ImagePtr> {
        let image = self.thumbnail(&data, width, height)?;

        Some(ImagePtr::new(&image))
    }

    fn read_with_format<F>(&self, data: &[u8], f: F) -> Option<Vec<u8>>
    where
        F: FnOnce(&mut image::DynamicImage, image::ImageFormat, &mut Vec<u8>),
    {
        let cursor = Cursor::new(data);

        if let Ok(reader) = Reader::new(cursor).with_guessed_format() {
            let format = reader.format()?;

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
