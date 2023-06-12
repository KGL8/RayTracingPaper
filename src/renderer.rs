use cgmath::{Vector2, Vector3};

pub fn per_pixel (coord: Vector2<f32>) -> [u8;4] {
    let mut rgb = Vector3::new(coord.x,coord.y,0.);
    
    let rgba_out: [u8; 4] = [
                            (rgb.x*255.) as u8,     //red
                            (rgb.y*255.) as u8,     //green
                            (rgb.z*255.) as u8,     //blue
                            255                     //alpha
                            ];
    rgba_out
}
