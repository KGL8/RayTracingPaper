use cgmath::Vector2;
use pixels::Pixels;
use crate::renderer::per_pixel;

use crate::app::{width, height, aspect_ratio};

pub fn draw_frame(pixels: &mut Pixels) -> Result<(), pixels::Error> {
    draw(pixels.frame_mut());
    pixels.render()
}

fn draw(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = ((i % width as usize) as f32)/(width as f32);                     // dividing by width normalizes
        let y = 1.-(((i / width as usize) as f32)/(height as f32));               // 1-y inverts
        let coord = Vector2::new((x*2.-1.)*aspect_ratio,y*2.-1.);    // *2-1 just maps 0->1 to -1->1 ; multiplying by aspect ratio fixes the setretching
        let rgba = per_pixel(coord);
        pixel.copy_from_slice(&rgba);
    }
}