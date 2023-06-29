use cgmath::{Vector3, Vector2, dot, InnerSpace};
use pixels::{Pixels, Error};
use winit_input_helper::WinitInputHelper;
use crate::{Camera, app::{WIDTH, HEIGHT}, utils::Ray};

#[derive(Copy, Clone)]
pub struct Renderer {
    window: Vector2<f32>,
    camera: Camera
}

impl Renderer {

    pub fn new (window: Vector2<f32>, camera: Camera) -> Renderer {
        Renderer {
            window,
            camera
        }
    }

    pub fn on_update (&mut self, timestep: f32, input: &WinitInputHelper) {
        self.camera.on_update(timestep, &input);
    }

    pub fn render (self, pixels: &mut Pixels) -> Result<(), Error> {

        Ok(for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = ((i % WIDTH as usize) as f32)/(WIDTH as f32);                        // dividing by width normalizes
            let y = 1.-(((i / WIDTH as usize) as f32)/(HEIGHT as f32));                  // 1-y inverts

            let ray_dir = Camera::get_ray_direction((x + y * WIDTH) as usize);
            let ray_origin = self.camera.get_camera_position();
            let ray = Ray{ origin: ray_origin, direction: ray_dir };
            //println!("{:?}",ray);
            let color = Self::trace_ray(ray);

            let rgba_out: [u8; 4] = [
                                    (color.x * 255.) as u8,     //red
                                    (color.y * 255.) as u8,     //green
                                    (color.z * 255.) as u8,     //blue
                                    255                         //alpha
                                    ];
            pixel.copy_from_slice(&rgba_out);
            return pixels.render();
        })
    }

    fn trace_ray(ray: Ray) -> Vector3<f32> {
        let mut rgb = Vector3::new(1.,0.,1.);

        let radius = 0.5;

        let a = dot(ray.direction,ray.direction);
        let b = 2. * dot(ray.origin,ray.direction);
        let c = dot(ray.origin,ray.origin) - radius * radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return Vector3::new(0.,0.,0.);
        }

        let t0 = (-b + discriminant.sqrt()) / (2. * a);
        let closest_t = (-b - discriminant.sqrt()) / (2. * a);

        let hit_point = ray.origin + ray.direction * closest_t;
        let normal = hit_point.normalize();

        let light_dir = Vector3::new(-1.0,-1.0,-1.0).normalize();
        let d = dot(normal,-light_dir).max(0.);

        return rgb * d;
    }

}
