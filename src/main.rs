use glium::{
    uniform,
    winit::event::{Event, WindowEvent},
    Surface,
};

mod teapot;

// #[derive(Copy, Clone, Debug)]
// struct Vertex {
//     position: [f32; 2],
//     color: [f32; 3],
// }
// implement_vertex!(Vertex, position, color);

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    // let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    let mut t: f32 = 0.0;
    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    t += 0.01;
                    // let x = t.sin() * 0.5;
                    // let c = t.cos();
                    // let s = t.sin();
                    let s = 0.01;
                    let uniforms = uniform! {
                        transform: [
                            [s, 0.0, 0.0, 0.0],
                            [0.0, s, 0.0, 0.0],
                            [0.0, 0.0, s, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ]
                    };
                    {
                        let mut frame = display.draw();
                        frame.clear_color(0.0, 0.0, 0.0, 1.0);

                        frame
                            .draw(
                                &positions,
                                &indices,
                                &program,
                                &uniforms,
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
    #version 150
    in vec3 position;
    // in vec3 color;
    // out vec3 vertex_color;

    uniform mat4 transform;
    
    void main() {
        // vertex_color = color;
        gl_Position = transform * vec4(position, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    #version 150
    // in vec3 vertex_color;
    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
