use image::{DynamicImage, ImageFormat};
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
    pub fn new(data: &Vec<u8>) -> Self {
        Self {
            mem_loc: data.as_ptr(),
            size: data.len(),
        }
    }
}

#[wasm_bindgen]
pub fn blur(data: &[u8], value: f32) -> Option<Vec<u8>> {
    read_with_format(&data, |img, format, bytes| {
        img.blur(value)
            .write_to(bytes, format)
            .expect("failed to blur image");
    })
}

#[wasm_bindgen]
pub fn blur_as_ptr(data: &[u8], value: f32) -> Option<ImagePtr> {
    let image = blur(&data, value)?;

    Some(ImagePtr::new(&image))
}

#[wasm_bindgen]
pub fn brighten(data: &[u8], value: i32) -> Option<Vec<u8>> {
    read_with_format(&data, |img, format, bytes| {
        img.brighten(value)
            .write_to(bytes, format)
            .expect("failed to brighten image");
    })
}

#[wasm_bindgen]
pub fn brighten_as_ptr(data: &[u8], value: i32) -> Option<ImagePtr> {
    let image = brighten(&data, value)?;

    Some(ImagePtr::new(&image))
}

#[wasm_bindgen]
pub fn grayscale(data: &[u8]) -> Option<Vec<u8>> {
    read_with_format(&data, |img, format, bytes| {
        img.grayscale()
            .write_to(bytes, format)
            .expect("failed to grayscale image");
    })
}

#[wasm_bindgen]
pub fn grayscale_as_ptr(data: &[u8]) -> Option<ImagePtr> {
    let image = grayscale(&data)?;

    Some(ImagePtr::new(&image))
}

#[wasm_bindgen]
pub fn invert(data: &[u8]) -> Option<Vec<u8>> {
    read_with_format(&data, |img, format, bytes| {
        img.invert();

        img.write_to(bytes, format).expect("failed to invert image");
    })
}

#[wasm_bindgen]
pub fn invert_as_ptr(data: &[u8]) -> Option<ImagePtr> {
    let image = invert(&data)?;

    Some(ImagePtr::new(&image))
}

#[wasm_bindgen]
pub fn thumbnail(data: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    read_with_format(&data, |img, format, bytes| {
        img.thumbnail(width, height)
            .write_to(bytes, format)
            .expect("failed to thumbnail image");
    })
}

#[wasm_bindgen]
pub fn thumbnail_as_ptr(data: &[u8], width: u32, height: u32) -> Option<ImagePtr> {
    let image = thumbnail(&data, width, height)?;

    Some(ImagePtr::new(&image))
}

fn read_with_format<F>(data: &[u8], f: F) -> Option<Vec<u8>>
where
    F: FnOnce(&mut DynamicImage, ImageFormat, &mut Vec<u8>),
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
