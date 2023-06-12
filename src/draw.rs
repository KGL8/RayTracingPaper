use cgmath::Vector2;
use pixels::Pixels;
use crate::renderer::per_pixel;

use crate::app::{WIDTH, HEIGHT};

pub fn draw_frame(pixels: &mut Pixels) -> Result<(), pixels::Error> {
    draw(pixels.frame_mut());
    pixels.render()
}

fn draw(frame: &mut [u8]) {
    // The imageer is kind of like a pirate.
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = ((i % WIDTH.usize()) as f32)/WIDTH.f32();
        let y = ((i / WIDTH.usize()) as f32)/HEIGHT.f32();

        let rgba = per_pixel(Vector2::new(x, y));
        pixel.copy_from_slice(&rgba);
    }
}