use glium::{
    implement_vertex, uniform,
    winit::event::{Event, WindowEvent},
    DrawParameters, Frame, Surface,
};

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
    let light = [1.4, 0.4, -0.7f32];
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
                    let model_transform = [
                        [s, 0.0, 0.0, 0.0],
                        [0.0, s, 0.0, 0.0],
                        [0.0, 0.0, s, 0.0],
                        [0.0, 0.0, 2.0, 1.0f32],
                    ];
                    let uniforms = uniform! {};

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

const VERTEX_SHADER: &'static str = r#"
    #version 150
    in vec2 position;

    
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    #version 140
    
    void main() {
        gl_FragColor = vec4(0.0, 0.0, 1.0, 1.0);
    }
"#;
