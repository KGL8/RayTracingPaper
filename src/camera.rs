use cgmath::{Vector4, Vector3, Vector2, Quaternion, Rotation3, Rad, Rotation, InnerSpace, SquareMatrix, Matrix4, Zero};
use once_cell::sync::Lazy;
use winit::event::VirtualKeyCode;
use std::{sync::Mutex, vec};
use winit_input_helper::WinitInputHelper;

use crate::{utils::{quaternion_cross, quaternion_normalize, create_look_at}, app::{width, height}};

static view: Lazy<Mutex<Matrix4<f32>>> = Lazy::new(|| Mutex::new(Matrix4::from_value(1.0)));
static inverse_view: Lazy<Mutex<Matrix4<f32>>> = Lazy::new(|| Mutex::new(Matrix4::from_value(1.0)));

//cached ray dirs
static rayDirections: Lazy<Mutex<Vec<Vector3<f32>>>> = Lazy::new(|| Mutex::new(vec![Vector3::zero(); (width * height) as usize ]));

static lastMousePos: Lazy<Mutex<Vector2<f32>>> = Lazy::new(|| Mutex::new(Vector2::zero()));


#[derive(Copy, Clone)]
pub struct Camera {
    VerticalFOV: f32,
    NearClip: f32,
    FarClip: f32,
    CamPos: Vector3<f32>,
    CamDir: Vector3<f32>
}

impl Camera {
    pub fn new(VFOV: f32,NC: f32,FC: f32) -> Camera {
        Camera {
            VerticalFOV: VFOV,
            NearClip: NC,
            FarClip: FC,
            CamPos: Vector3::new(0.,0.,-1.),
            CamDir: Vector3::new(0.,0.,3.)
        }
    }

    pub fn on_update(mut self, timestep: f32, input: &WinitInputHelper) -> Vec<bool> {

        println!("\n[timestep: {:?}]", timestep);

        let (x, y) = input.mouse().unwrap_or((width / 2.0, height / 2.0));
        let mousePos = Vector2::new(x, y);
        let delta = (mousePos - *lastMousePos.lock().unwrap()) * 0.002;
        *lastMousePos.lock().unwrap() = mousePos;

        println!("\n[mouse position: {:?}]", mousePos);

        if input.mouse_held(1) != true {
            let letCursorMove = true;
            let hideCursor = false;
            
            println!("\n[mouse down]");

            return vec![letCursorMove, hideCursor];
        }

        let letCursorMove = false;
        let hideCursor = true;

        let mut moved = false;

        let upDirection = Vector3::new(0.0,1.0,0.0);
        let rightDirection = self.CamDir.cross(upDirection);

        let speed = 5.0;

        if input.key_held(VirtualKeyCode::W) {
            self.CamPos = self.CamPos + (self.CamDir * speed * timestep);
            moved = true;
            println!("\n[key: W]");
        } else if input.key_held(VirtualKeyCode::S) {
            self.CamPos = self.CamPos - (self.CamDir * speed * timestep);
            moved = true;
            println!("\n[key: S]");
        }

        if input.key_held(VirtualKeyCode::A) {
            self.CamPos = self.CamPos - (rightDirection * speed * timestep);
            moved = true;
            println!("\n[key: A]");
        } else if input.key_held(VirtualKeyCode::D) {
            self.CamPos = self.CamPos + (rightDirection * speed * timestep);
            moved = true;
            println!("\n[key: D]");
        }

        if input.key_held(VirtualKeyCode::Q) {
            self.CamPos = self.CamPos - (upDirection * speed * timestep);
            moved = true;
            println!("\n[key: Q]");
        } else if input.key_held(VirtualKeyCode::E) {
            self.CamPos = self.CamPos + (upDirection * speed * timestep);
            moved = true;
            println!("\n[key: E]");
        }

        if delta.x != 0.0 || delta.y != 0.0 {
            let pitchDelta = delta.y * 0.3;
            let yawDelta = delta.x * 0.3;

            let rotationPitch = Quaternion::from_axis_angle(rightDirection,-Rad(pitchDelta));
            let rotationYaw = Quaternion::from_axis_angle(Vector3::new(0.0,1.0,0.0),-Rad(yawDelta));

            let q = quaternion_normalize(quaternion_cross(rotationPitch, rotationYaw));
            self.CamDir = q.rotate_vector(self.CamDir);

            moved = true;
        }

        if moved {
            self.recalculate_view();
            self.recalculate_ray_directions();
        }

        println!("[ Camera Position: {:?} ]", self.CamPos); //not changing why??
        return vec![letCursorMove, hideCursor];
    }

    pub fn recalculate_view(self) {
        let mut view_lock = view.lock().unwrap();
        *view_lock = create_look_at(self.CamPos, self.CamPos + self.CamDir, Vector3::new(0., 1., 0.));
    
        if let Some(matrix) = view_lock.invert() {
            *inverse_view.lock().unwrap() = matrix;
        } else {
            return;
        }
    }
    

    pub fn recalculate_ray_directions(self) {
        
        for y in 0..(height as i32) {
            for x in 0..(width as i32) {
                let mut coord = Vector2::new(x as f32 / width, y as f32 / height);
                coord = coord * 2.0 - Vector2::new(1.0,1.0); // -1 -> 1

                let target = Vector4::new(coord.x, coord.y, 1.0, 1.0);
                let step_2 = (Vector3::new(target.x, target.y, target.z) / target.w).normalize();
                let step_3 = *inverse_view.lock().unwrap() * Vector4::new(step_2.x, step_2.y, step_2.z, 0.);
                let rayDirection = Vector3::new(step_3.x, step_3.y, step_3.z);
                (*rayDirections.lock().unwrap())[(x + y + width as i32) as usize] = rayDirection;
            }
        }
    }
}