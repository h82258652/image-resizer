mod utils;

use std::io::Cursor;

use image::codecs::gif::{GifEncoder, Repeat};
use image::{codecs::gif::GifDecoder, imageops::FilterType, io::Reader, ImageFormat};
use image::{AnimationDecoder, Frame};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Size(pub u32, pub u32);

#[wasm_bindgen]
pub fn get_size(data: &[u8]) -> Size {
    let reader = Reader::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap();

    let image = reader.decode().unwrap();
    Size(image.width(), image.height())
}

#[wasm_bindgen]
pub fn resize(data: &[u8], width: u32, height: u32) -> Uint8Array {
    let reader = Reader::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap();

    let format = reader.format().unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    if format == ImageFormat::Gif {
        let image = reader.decode().unwrap();
        let original_width = image.width();
        let original_height = image.height();

        let decoder = GifDecoder::new(data).unwrap();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().unwrap();
        let mut new_frames = Vec::new();
        for frame in &frames {
            let buffer = frame.buffer();

            let new_frame_width = buffer.width() * width / original_width;
            let new_frame_height = buffer.height() * height / original_height;
            let new_frame_left = frame.left() * width / original_width;
            let new_frame_top = frame.top() * height / original_height;

            let new_buffer = image::imageops::resize(
                buffer,
                new_frame_width,
                new_frame_height,
                FilterType::Triangle,
            );
            new_frames.push(Frame::from_parts(
                new_buffer,
                new_frame_left,
                new_frame_top,
                frame.delay(),
            ));
        }

        let mut encoder = GifEncoder::new(&mut bytes);
        encoder.set_repeat(Repeat::Infinite).unwrap();
        encoder.encode_frames(new_frames.into_iter()).unwrap();
    } else {
        let mut image = reader.decode().unwrap();
        image = image.resize_exact(width, height, FilterType::Triangle);

        image.write_to(&mut cursor, format).unwrap();
    }

    let length = bytes.len() as u32;
    let result = Uint8Array::new_with_length(length);
    result.copy_from(&bytes);
    result
}
