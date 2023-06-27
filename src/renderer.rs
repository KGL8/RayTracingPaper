use cgmath::{Vector3, Vector2, dot, num_traits::pow, InnerSpace};
use pixels::{Pixels, Error};
use winit_input_helper::WinitInputHelper;
use crate::{Camera, app::{width, height, aspect_ratio}};

#[derive(Copy, Clone)]
pub struct Renderer {
    window: Vector2<f32>,
    camera: Camera
}

impl Renderer {

    pub fn new (Widow: Vector2<f32>, Camera: Camera) -> Renderer {
        Renderer {
            window: Widow,
            camera: Camera
        }
    }

    pub fn on_update (self, timestep: f32, input: &WinitInputHelper) {
        self.camera.on_update(timestep, &input);
    }

    pub fn render (self, pixels: &mut Pixels) -> Result<(), Error> {

        Ok(for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = ((i % width as usize) as f32)/(width as f32);                     // dividing by width normalizes
            let y = 1.-(((i / width as usize) as f32)/(height as f32));               // 1-y inverts
            let coord = Vector2::new((x*2.-1.)*aspect_ratio,y*2.-1.);      // *2-1 just maps 0->1 to -1->1 ; multiplying by aspect ratio fixes the setretching
            let mut rgb = Vector3::new(1.,0.,1.);

            let ray_dir = Vector3::new(coord.x, coord.y, -1.);
            let ray_origin = Vector3::new(0.,0.,1.);
            let radius = 0.5;

            let a = dot(ray_dir,ray_dir);
            let b = 2. * dot(ray_origin,ray_dir);
            let c = dot(ray_origin,ray_origin) - pow(radius,2);

            let discriminant = pow(b,2) - 4. * a * c;

            if discriminant < 0. {
                rgb = Vector3::new(0.,0.,0.);
            }

            let t0 = (-b + discriminant.sqrt()) / (2. * a);
            let closest_t = (-b - discriminant.sqrt()) / (2. * a);

            let hit_point = ray_origin + ray_dir * closest_t;
            let normal = hit_point.normalize();

            let light_dir = Vector3::new(-1.0,-1.0,-1.0).normalize();

            let d = dot(normal,-light_dir).max(0.);

            rgb = rgb * d;

            let rgba_out: [u8; 4] = [
                                    (rgb.x * 255.) as u8,     //red
                                    (rgb.y * 255.) as u8,     //green
                                    (rgb.z * 255.) as u8,     //blue
                                    255                       //alpha
                                    ];
            pixel.copy_from_slice(&rgba_out);
            return pixels.render();
        })
    }

}
