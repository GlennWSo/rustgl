use glium::{implement_vertex, Surface};

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

impl From<[f32; 2]> for Vertex {
    fn from(value: [f32; 2]) -> Self {
        Self { position: value }
    }
}

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);

    let shape: [Vertex; 3] = [
        [-0.5, -0.5], // vertex1
        [-0.0, 0.5],
        [0.5, -0.25],
    ]
    .map(|v| v.into());
    let vertex_buff = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140
        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
    frame
        .draw(
            &vertex_buff,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    frame.finish().unwrap();

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
