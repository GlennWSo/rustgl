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
                    let size = frame.get_dimensions();
                    let uniforms = uniform! {
                        u_resolution: [size.0 as f32, size.1 as f32],
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

const VERTEX_SHADER: &'static str = r#"
    #version 150
    in vec2 position;
    // uniform vec2 u_resolution;

    
    void main() {

        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    #version 140
    uniform vec2 u_resolution;
    
    void main() {
        vec2 st = fract(gl_FragCoord.xy/u_resolution*1.0);
        float maxd = max(u_resolution.x, u_resolution.y);
        vec2 pos = vec2(0.5) - st;
        pos.x *= u_resolution.x / u_resolution.y;
        float r = length(pos);
        float inside = step(0.5, r);
        vec3 spectra = vec3(st, 1.0);
        vec3 color = mix(vec3(0.0), spectra, inside);


        gl_FragColor = vec4(color, 1.0);
    }
"#;
