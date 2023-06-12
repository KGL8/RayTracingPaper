

use cgmath::{Vector2, Vector3, InnerSpace, num_traits::pow};

pub fn per_pixel (coord: Vector2<f32>) -> [u8;4] {
    let mut rgb = Vector3::new(1.,0.,1.);
    
    let ray_dir = Vector3::new(coord.x, coord.y, -1.);
    let ray_origin = Vector3::new(0.,0.,1.);
    let radius = 0.5;

    let a = ray_dir.dot(ray_dir);
    let b = 2. * ray_origin.dot(ray_dir);
    let c = ray_origin.dot(ray_origin) - pow(radius,2);

    let discriminant = pow(b,2) - 4. * a * c;

    if discriminant < 0. {
        rgb = Vector3::new(0.,0.,0.);
    }

    let t0 = (-b + discriminant.sqrt()) / (2. * a);
    let closest_t = (-b - discriminant.sqrt()) / (2. * a);

    let hit_point = ray_origin + ray_dir * closest_t;
    let normal = hit_point.normalize();

    let light_dir = Vector3::new(-1.,-1.,-1.).normalize();

    let d = normal.dot(-light_dir).max(0.);

    rgb *= d;

    let rgba_out: [u8; 4] = [
                            (rgb.x * 255.) as u8,     //red
                            (rgb.y * 255.) as u8,     //green
                            (rgb.z * 255.) as u8,     //blue
                            255                       //alpha
                            ];
    rgba_out
}
