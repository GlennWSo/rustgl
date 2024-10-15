use glium::{
    implement_vertex, uniform,
    winit::event::{Event, WindowEvent},
    DrawParameters, Frame, Surface,
};

use nalgebra::{self as na, Point3, Vector3};
use rustgl::perspective;

type Mat4 = na::Matrix4<f32>;
type Vec3 = na::Vector3<f32>;

const VERTEX_SHADER: &'static str = r#"
    #version 150
    in vec2 position;
    // uniform vec2 u_resolution;

    
    void main() {

        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = include_str!("sphere.frag");

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let tri = [
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [3.0, -1.0],
        },
        Vertex {
            position: [-1.0, 3.0],
        },
    ];

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    let mut t: f32 = 0.0;
    let mut mouse = [0.0, 0_f32];
    // let near = 1.0;
    // let diag = Vec3::new(near, near, 0.0);

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    mouse[0] = position.x as f32;
                    mouse[1] = position.y as f32;
                }
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    let mut frame = display.draw();
                    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                    t += 0.01;
                    // let x = t.sin() * 0.5;
                    // let c = t.cos();
                    // let s = t.sin();
                    let s = 0.01;
                    let dims = frame.get_dimensions();
                    let camera_k: Mat4 = perspective(dims.0, dims.1).into();
                    let camera_origin = Point3::new(1.0, 0.0, 1_f32);
                    let camera_target = Point3::new(0.0, 0.0, 0_f32);
                    let up = &Vector3::y();
                    let camera_rt = Mat4::look_at_rh(&camera_origin, &camera_target, up);
                    // let model_transform = [
                    //     [s, 0.0, 0.0, 0.0],
                    //     [0.0, s, 0.0, 0.0],
                    //     [0.0, 0.0, s, 0.0],
                    //     [0.0, 0.0, 2.0, 1.0f32],
                    // ];
                    let camera_inv = (camera_k * camera_rt).try_inverse().unwrap();
                    let raw_camera = camera_inv.data.0;

                    let uniforms = uniform! {
                        u_resolution: [dims.0 as f32, dims.1 as f32],
                        u_time: t,
                        u_mouse: mouse,
                        u_camera: raw_camera,
                    };

                    let draw_parameters = DrawParameters {
                        // depth: glium::Depth {
                        //     test: glium::DepthTest::IfLess,
                        //     write: true,
                        //     ..Default::default()
                        // },
                        ..Default::default()
                    };

                    let vertex_buff = glium::VertexBuffer::new(&display, &tri).unwrap();
                    let no_indices =
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
                    frame
                        .draw(
                            &vertex_buff,
                            &no_indices,
                            &program,
                            &uniforms,
                            &draw_parameters,
                        )
                        .unwrap();
                    frame.finish().unwrap();
                }
                WindowEvent::Resized(size) => display.resize(size.into()),
                _ => (),
            },
            Event::AboutToWait => window.request_redraw(),
            _ => (),
        };
    });
}
