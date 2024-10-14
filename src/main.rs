use glium::{
    uniform,
    winit::event::{Event, WindowEvent},
    DrawParameters, Frame, Surface,
};

mod teapot;

// #[derive(Copy, Clone, Debug)]
// struct Vertex {
//     position: [f32; 2],
//     color: [f32; 3],
// }
// implement_vertex!(Vertex, position, color);

fn perspective(frame: &Frame) -> [[f32; 4]; 4] {
    let (width, height) = frame.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    let light = [-1.0, 0.4, 0.9f32];
    let mut t: f32 = 0.0;

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    let mut frame = display.draw();
                    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                    t += 0.01;
                    // let x = t.sin() * 0.5;
                    // let c = t.cos();
                    // let s = t.sin();
                    let s = 0.01;
                    let transform = [
                        [s, 0.0, 0.0, 0.0],
                        [0.0, s, 0.0, 0.0],
                        [0.0, 0.0, s, 0.0],
                        [0.0, 0.0, 2.0, 1.0f32],
                    ];
                    let uniforms = uniform! {
                        u_light: light,
                        transform: transform,
                        perspective: perspective(&frame),
                    };

                    let draw_parameters = DrawParameters {
                        depth: glium::Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    };

                    frame
                        .draw(
                            (&positions, &normals),
                            &indices,
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

const VERTEX_SHADER: &'static str = r#"
    #version 150
    in vec3 position;
    in vec3 normal;
    out vec3 v_normal;

    uniform mat4 transform;
    uniform mat4 perspective;
    
    void main() {
        // vertex_color = color;
        v_normal = normalize(transpose(inverse(mat3(transform))) * normal);
        gl_Position = perspective*transform * vec4(position, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    #version 140
    in vec3 v_normal;
    out vec4 color;
    uniform vec3 u_light;

    void main() {
        float light = dot(u_light, v_normal);
        vec3 full_color = vec3(1.0, 0.0, 0.0);
        vec3 dark_color = mix(vec3(0.0), full_color, 0.6);
        color = vec4(mix(dark_color, full_color, light), 1.0);
    }
"#;
