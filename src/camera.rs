use cgmath::{Vector4, Vector3, Vector2, Quaternion, Rotation3, Rad, Rotation, InnerSpace, SquareMatrix, Matrix4, Zero, Point3};
use once_cell::sync::Lazy;
use winit::event::VirtualKeyCode;
use std::{sync::Mutex, vec};
use winit_input_helper::WinitInputHelper;

use crate::{utils::{quaternion_cross, quaternion_normalize}, app::{WIDTH, HEIGHT}};

static VIEW: Lazy<Mutex<Matrix4<f32>>> = Lazy::new(|| Mutex::new(Matrix4::from_value(1.0)));
static INVERSE_VIEW: Lazy<Mutex<Matrix4<f32>>> = Lazy::new(|| Mutex::new(Matrix4::from_value(1.0)));

//cached ray dirs
static RAY_DIRECTIONS: Lazy<Mutex<Vec<Vector3<f32>>>> = Lazy::new(|| Mutex::new(vec![Vector3::zero(); (WIDTH * HEIGHT) as usize ]));

static LAST_MOUSE_POS: Lazy<Mutex<Vector2<f32>>> = Lazy::new(|| Mutex::new(Vector2::zero()));


#[derive(Copy, Clone)]
pub struct Camera {
    vertical_fov: f32,
    near_clip: f32,
    far_clip: f32,
    cam_pos: Vector3<f32>,
    cam_dir: Vector3<f32>
}

impl Camera {
    pub fn new(vfov: f32,nc: f32,fc: f32) -> Camera {
        Camera {
            vertical_fov: vfov,
            near_clip: nc,
            far_clip: fc,
            cam_pos: Vector3::new(0.,0.,-1.),
            cam_dir: Vector3::new(0.,0.,3.)
        }
    }

    pub fn on_update(&mut self, timestep: f32, input: &WinitInputHelper) -> Vec<bool> {

        //println!("\n[timestep: {:?}]", timestep);

        let (x, y) = input.mouse().unwrap_or((WIDTH / 2.0, HEIGHT / 2.0));
        let mouse_pos = Vector2::new(x, y);
        let delta = (mouse_pos - *LAST_MOUSE_POS.lock().unwrap()) * 0.002;
        *LAST_MOUSE_POS.lock().unwrap() = mouse_pos;

        //println!("[mouse position: {:?}]", mousePos);

        let mut let_cursor_move = true;
        let mut hide_cursor = false;
        let mut moved = false;

        let up_direction = Vector3::new(0.0,1.0,0.0);
        let right_direction = self.cam_dir.cross(up_direction);

        if input.mouse_held(1) == true {
            let_cursor_move = false;
            hide_cursor = true;
            
            if delta.x != 0.0 || delta.y != 0.0 {
                let pitch_delta = delta.y * 0.3;
                let yaw_delta = delta.x * 0.3;
    
                let rotation_pitch = Quaternion::from_axis_angle(right_direction,-Rad(pitch_delta));
                let rotation_yaw = Quaternion::from_axis_angle(Vector3::new(0.0,1.0,0.0),-Rad(yaw_delta));
    
                let q = quaternion_normalize(quaternion_cross(rotation_pitch, rotation_yaw));
                self.cam_dir = q.rotate_vector(self.cam_dir);
    
                moved = true;
            }
            //println!("[mouse down]");
        }

        let speed = 5.0;

        if input.key_held(VirtualKeyCode::W) {
            self.cam_pos = self.cam_pos + (self.cam_dir * speed * timestep);
            moved = true;
            //println!("[key: W]");
        } else if input.key_held(VirtualKeyCode::S) {
            self.cam_pos = self.cam_pos - (self.cam_dir * speed * timestep);
            moved = true;
            //println!("[key: S]");
        }

        if input.key_held(VirtualKeyCode::A) {
            self.cam_pos = self.cam_pos - (right_direction * speed * timestep);
            moved = true;
            //println!("[key: A]");
        } else if input.key_held(VirtualKeyCode::D) {
            self.cam_pos = self.cam_pos + (right_direction * speed * timestep);
            moved = true;
            //println!("[key: D]");
        }

        if input.key_held(VirtualKeyCode::Q) {
            self.cam_pos = self.cam_pos - (up_direction * speed * timestep);
            moved = true;
            //println!("\n[key: Q]");
        } else if input.key_held(VirtualKeyCode::E) {
            self.cam_pos = self.cam_pos + (up_direction * speed * timestep);
            moved = true;
            //println!("\n[key: E]");
        }

        if moved {
            self.recalculate_view();
            self.recalculate_ray_directions();
        }

        // println!("[Camera Position: {:?}]", self.CamPos);
        // println!("[Camera Direction: {:?}]", self.CamDir);
        return vec![let_cursor_move, hide_cursor];
    }

    pub fn recalculate_view(self) {
        let mut view_lock = VIEW.lock().unwrap();
        *view_lock = Matrix4::look_to_rh(Point3::new(self.cam_pos.x, self.cam_pos.y, self.cam_pos.z), self.cam_pos + self.cam_dir, Vector3::new(0., 1., 0.));
        println!("view: {:?}",view_lock);
        if let Some(matrix) = view_lock.invert() {
            *INVERSE_VIEW.lock().unwrap() = matrix;
            println!("inverse: {:?}",matrix);
        } else {
            return;
        }
    }
    

    pub fn recalculate_ray_directions(self) {
        
        for y in 0..(HEIGHT as i32) {
            for x in 0..(WIDTH as i32) {
                let mut coord = Vector2::new(x as f32 / WIDTH, y as f32 / HEIGHT);
                coord = coord * 2.0 - Vector2::new(1.0,1.0); // -1 -> 1

                let target = Vector4::new(coord.x, coord.y, 1.0, 1.0);
                let step_2 = (Vector3::new(target.x, target.y, target.z) / target.w).normalize();
                let step_3 = *INVERSE_VIEW.lock().unwrap() * Vector4::new(step_2.x, step_2.y, step_2.z, 0.);
                let ray_direction = Vector3::new(step_3.x, step_3.y, step_3.z);
                (*RAY_DIRECTIONS.lock().unwrap())[(x + y + WIDTH as i32) as usize] = ray_direction;
            }
        }
    }

    pub fn get_ray_direction(i: usize) -> Vector3<f32>{
        (*RAY_DIRECTIONS.lock().unwrap())[i]
    }

    pub fn get_camera_position(&self) -> Vector3<f32>{
        self.cam_pos
    }
}