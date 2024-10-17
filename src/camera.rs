use nalgebra::{self as na, Matrix4, Vector3};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub type Point3 = na::Point3<f32>;
pub type Vec3 = Vector3<f32>;
pub type Mat4 = Matrix4<f32>;

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

struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }
    /// returns true if mutation happens
    fn process_events(&mut self, event: &WindowEvent) -> bool {
        let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(key),
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } = event
        else {
            return false;
        };
        match key {
            KeyCode::ArrowUp | KeyCode::KeyW => {
                self.is_forward_pressed = true;
                true
            }
            _ => false,
        }
    }
}
