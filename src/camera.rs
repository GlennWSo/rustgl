use nalgebra::{self as na, Matrix4, Vector3};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub type Point3 = na::Point3<f32>;
pub type Vec3 = Vector3<f32>;
pub type Mat4 = Matrix4<f32>;

#[derive(Debug)]
pub struct PerspectiveCamera {
    pub eye: Point3,
    pub target: Point3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}
/// The coordinate system in Wgpu is based on DirectX and Metal's coordinate systems. That
/// means that in normalized device coordinates (opens new window), the x-axis and y-axis
/// are in the range of -1.0 to +1.0, and the z-axis is 0.0 to +1.0. The cgmath crate (as
/// well as most game math crates) is built for OpenGL's coordinate system. This matrix will
/// scale and translate our scene from OpenGL's coordinate system to WGPU's. We'll define it
/// as follows.
/// NOTE: nalgebra is row major while gpu mats are column major
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl PerspectiveCamera {
    pub fn view_projection(&self) -> Mat4 {
        let view = Mat4::look_at_rh(&self.eye, &self.target, &self.up);
        let proj = Mat4::new_perspective(self.aspect, self.fovy, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::NoUninit)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: Mat4::identity().into(),
        }
    }
}

impl CameraUniform {
    pub fn update(&mut self, camera: &PerspectiveCamera) {
        self.view_proj = camera.view_projection().into()
    }
}

#[derive(Debug)]
pub struct CameraController {
    speed: f32,
    /// is this input set
    is_up: bool,
    /// is this input set
    is_down: bool,
    /// is this input set
    is_left: bool,
    /// is this input set
    is_right: bool,
    pub camera: PerspectiveCamera,
}

impl CameraController {
    pub fn new(speed: f32, camera: PerspectiveCamera) -> Self {
        Self {
            speed,
            is_up: false,
            is_down: false,
            is_left: false,
            is_right: false,
            camera,
        }
    }

    fn is_unset(&mut self) -> bool {
        ![self.is_up, self.is_down, self.is_left, self.is_right]
            .into_iter()
            .any(|e| e)
    }
    pub fn update_camera(&mut self, time_step: f32) {
        if self.is_unset() {
            return;
        }
        let step = self.speed * time_step;
        let normal = (self.camera.target - self.camera.eye).normalize();
        let dr = match (self.is_up, self.is_down) {
            (true, false) => step,
            (_, true) => -step,
            _ => 0.0,
        };
        self.camera.eye += dr * normal;
    }

    /// returns true if mutation happens
    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(key),
                    state,
                    ..
                },
            ..
        } = event
        else {
            return false;
        };
        let pressed = match state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };
        match key {
            KeyCode::ArrowUp | KeyCode::KeyW => {
                self.is_up = pressed;
                true
            }
            KeyCode::ArrowLeft | KeyCode::KeyA => {
                self.is_left = pressed;
                true
            }
            KeyCode::ArrowRight | KeyCode::KeyD => {
                self.is_right = pressed;
                true
            }
            KeyCode::ArrowDown | KeyCode::KeyS => {
                self.is_down = pressed;
                true
            }
            _ => false,
        }
    }
}
