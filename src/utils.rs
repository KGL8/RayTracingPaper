use cgmath::{Quaternion, Vector3, Matrix4, dot, InnerSpace};
use error_iter::ErrorIter;
use log::error;

pub fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
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

pub fn create_look_at<T: cgmath::BaseFloat>(
    eye: Vector3<T>,
    center: Vector3<T>,
    up: Vector3<T>,
) -> Matrix4<T> {
    let f = (center - eye).normalize();
    let s = up.cross(f).normalize();
    let u = f.cross(s);

    Matrix4::new(
        s.x, u.x, -f.x, T::zero(),
        s.y, u.y, -f.y, T::zero(),
        s.z, u.z, -f.z, T::zero(),
        -dot(s, eye), -dot(u, eye), dot(f, eye), T::one(),
    )
}