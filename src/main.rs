use glium::{
    implement_vertex, uniform,
    winit::event::{Event, WindowEvent},
    Surface,
};

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
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let shape: [Vertex; 3] = [
        [-0.5, -0.5], // vertex1
        [-0.0, 0.5],
        [0.5, -0.25],
    ]
    .map(|v| v.into());
    let vertex_buff = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    let mut t: f32 = 0.0;

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let offset = t.sin() * 0.5;
                    {
                        let offset = offset;
                        let mut frame = display.draw();
                        frame.clear_color(0.0, 0.0, 0.0, 1.0);

                        frame
                            .draw(
                                &vertex_buff,
                                &indices,
                                &program,
                                &uniform! {x: offset},
                                &Default::default(),
                            )
                            .unwrap();
                        frame.finish().unwrap();
                    }
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
    #version 140
    in vec2 position;

    uniform float x;
    
    void main() {
        vec2 pos = position;
        pos.x += x;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    #version 140
    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
fn draw_frame(offset: f32, display: &glium::Display<glium::glutin::surface::WindowSurface>) {
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);

    let shape: [Vertex; 3] = [
        [-0.5, -0.5], // vertex1
        [-0.0, 0.5],
        [0.5, -0.25],
    ]
    .map(|v| v.into());
    let vertex_buff = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    frame
        .draw(
            &vertex_buff,
            &indices,
            &program,
            &uniform! {x: offset},
            &Default::default(),
        )
        .unwrap();
    frame.finish().unwrap();
}
