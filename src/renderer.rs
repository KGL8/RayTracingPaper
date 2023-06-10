use pixels::Pixels;
use rand::{Rng, thread_rng};

use crate::app::{WIDTH, HEIGHT};

pub fn draw_frame(pixels: &mut Pixels) -> Result<(), pixels::Error> {
    draw(pixels.frame_mut());
    pixels.render()
}

fn draw(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % WIDTH.usize();
        let y = i / WIDTH.usize();

        let rgba = calculate_color(x, y);
        pixel.copy_from_slice(&rgba);
    }
}

fn calculate_color(x: usize, y: usize) -> [u8; 4] {
    let x_ratio = x as f32 / (WIDTH.f32() - 1.) as f32;
    let y_ratio = y as f32 / (HEIGHT.f32() - 1.) as f32;

    let r = (x_ratio * 255.0) as u8;
    let g = (y_ratio * 255.0) as u8;
    let b = 50;
    let a = 255;

    let rgba: [u8; 4] = [r, g, b, a];
    
    rgba
}