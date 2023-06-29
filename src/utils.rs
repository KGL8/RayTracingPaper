use cgmath::{Quaternion, Vector3};
use error_iter::ErrorIter;
use log::error;

#[derive(Debug)]
pub struct Ray {
    pub(crate) origin: Vector3<f32>,
    pub(crate) direction: Vector3<f32>
}

pub fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{}() failed: {}", method_name, err);
    let mut source = Some(&err as &(dyn std::error::Error + 'static));
    while let Some(err) = source {
        if let Some(next_source) = err.source() {
            error!("  Caused by: {}", next_source);
            source = Some(next_source);
        } else {
            break;
        }
    }
}

pub fn quaternion_cross<T: cgmath::BaseFloat>(
    q1: Quaternion<T>,
    q2: Quaternion<T>,
) -> Quaternion<T> {
    Quaternion::new(
        q1.s * q2.s - q1.v.x * q2.v.x - q1.v.y * q2.v.y - q1.v.z * q2.v.z,
        q1.s * q2.v.x + q1.v.x * q2.s + q1.v.y * q2.v.z - q1.v.z * q2.v.y,
        q1.s * q2.v.y + q1.v.y * q2.s + q1.v.z * q2.v.x - q1.v.x * q2.v.z,
        q1.s * q2.v.z + q1.v.z * q2.s + q1.v.x * q2.v.y - q1.v.y * q2.v.x,
    )
}

pub fn quaternion_normalize<T: cgmath::BaseFloat>(
    q: Quaternion<T>,
) -> Quaternion<T> {

    let d = (q.s * q.s + q.v.x * q.v.x + q.v.y * q.v.y + q.v.z * q.v.z).sqrt();
    Quaternion::new( q.s / d ,q.v.x / d , q.v.y / d , q.v.z / d )
}