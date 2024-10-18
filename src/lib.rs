mod camera;
mod screen;
mod texture;

use std::mem::size_of;

use camera::{CameraController, Mat4, Mat4Uniform, PerspectiveCamera, Vec3};
use log::info;
use nalgebra::UnitQuaternion;
use screen::Screen;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt as _, RenderEncoder},
    BindGroup, BindGroupLayout, BufferUsages, PipelineCompilationOptions, PipelineLayout,
    RenderPipeline, ShaderModule, Texture, VertexAttribute, VertexBufferLayout,
};
use winit::{
    event::*,
    event_loop::{self, ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{Key, NamedKey},
    window::{Window, WindowBuilder},
};
type Position = [f32; 3];
type TexCoord = [f32; 2];

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::NoUninit)]
struct Vertex {
    position: Position,
    tex_coord: TexCoord,
}

impl From<(Position, TexCoord)> for Vertex {
    fn from(value: (Position, TexCoord)) -> Self {
        Vertex {
            position: value.0,
            tex_coord: value.1,
        }
    }
}

impl Vertex {
    const ATTRS: [VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241,   0.49240386, 0.0], tex_coord: [0.4131759,    0.99240386], }, // A
    Vertex { position: [-0.49513406,  0.06958647, 0.0], tex_coord: [0.0048659444, 0.56958647], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coord: [0.28081453,   0.05060294], }, // C
    Vertex { position: [0.35966998,   -0.3473291, 0.0], tex_coord: [0.85967,       0.1526709], }, // D
    Vertex { position: [0.44147372,    0.2347359, 0.0], tex_coord: [0.9414737,     0.7347359], }, // E
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4
];

struct TextureBundle {}

struct State<'a> {
    screen: Screen<'a>,
    // surface: wgpu::Surface<'a>,
    // device: wgpu::Device,
    // queue: wgpu::Queue,
    // config: wgpu::SurfaceConfiguration,
    active_pipeline: RenderPipeline,
    // bench_pipeline: RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: BindGroup,
    diffuse_texture: texture::Texture,
    camera_ctrl: CameraController,
    camera_uniform: Mat4Uniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    iso1: Isometry,
    // iso_uniform1: Mat4Uniform,
    iso_buffer: wgpu::Buffer,
    iso_bind_group: wgpu::BindGroup,
}

fn mk_pipeline<'a>(
    shader: &'a ShaderModule,
    layout: &'a PipelineLayout,
    color_targets: &'a [Option<wgpu::ColorTargetState>],
    vs_entry: &'static str,
    vertex_layout: &'a [VertexBufferLayout<'a>],
) -> wgpu::RenderPipelineDescriptor<'a> {
    wgpu::RenderPipelineDescriptor {
        label: Some("Awsome Pipe"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: vs_entry,
            compilation_options: PipelineCompilationOptions::default(),
            buffers: vertex_layout,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: PipelineCompilationOptions::default(),
            targets: color_targets,
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    }
}

/// rot first, then translate
#[derive(Default)]
struct Isometry {
    position: Vec3,
    rotation: UnitQuaternion<f32>,
}
impl Isometry {
    fn translation(&self) -> Mat4 {
        let mut mat4 = Mat4::identity();
        mat4[(0, 3)] = self.position[0];
        mat4[(1, 3)] = self.position[1];
        mat4[(2, 3)] = self.position[2];
        mat4
    }
    fn to_matrix(&self) -> Mat4 {
        let rot = self.rotation.to_homogeneous();
        // dbg!(UnitQuaternion::<f32>::identity());
        // dbg!(self.rotation);
        dbg!(self.translation() * rot)
    }
}

impl<'a> State<'a> {
    // Creating some of the wgpu types requires async code
    async fn new(window: &'a Window) -> State<'a> {
        // surface.configure(&device, &config);
        let screen = Screen::new(window).await;
        let (diffuse_texture, diffuse_bind_group, texture_bind_group_layout) =
            texture::config_texture(&screen);

        let shader = screen
            .device
            .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let buffer_desc = BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        };
        let vertex_buffer = screen.device.create_buffer_init(&buffer_desc);
        let index_buffer = screen.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsages::INDEX,
        });

        let camera = PerspectiveCamera {
            eye: [0.0, 1.0, 2.0].into(),
            target: [0.0, 0.0, 0.0].into(),
            up: Vec3::y(),
            aspect: screen.config.width as f32 / screen.config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };
        // let mut camera_uniform = Mat4Uniform::default();
        // camera_uniform.update(&camera);
        let camera_uniform: Mat4Uniform = camera.view_projection().into();
        let camera_ctrl = CameraController::new(1.0, camera);
        let camera_buffer = screen.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            screen
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("camera_bind_group_layout"),
                });
        let camera_bind_group = screen.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let iso = Isometry::default();
        let iso_uniform: Mat4Uniform = iso.to_matrix().into();
        let iso_buffer = screen.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[iso_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let iso_group_layout =
            screen
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("iso_group_layout"),
                });
        let iso_bind_group = screen.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &iso_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let pipeline_layout =
            screen
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &texture_bind_group_layout,
                        &camera_bind_group_layout,
                        &iso_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let vertex_layout = &[Vertex::desc()];
        let frag_color_target = [Some(wgpu::ColorTargetState {
            // 4.
            format: screen.config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];
        let main_desc = mk_pipeline(
            &shader,
            &pipeline_layout,
            &frag_color_target,
            "vs_main",
            vertex_layout,
        );
        let main_pipeline = screen.device.create_render_pipeline(&main_desc);

        Self {
            // window,
            screen,
            active_pipeline: main_pipeline,
            // bench_pipeline: rainbow_pipeline,
            vertex_buffer,
            index_buffer,
            diffuse_bind_group,
            diffuse_texture,
            camera_ctrl,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            iso1: iso,
            iso_buffer,
            iso_bind_group,
        }
    }

    // pub fn window(&self) -> &Window {
    //     &self.window
    // }

    /// returns true if mutation happens
    fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_ctrl.process_events(event)
    }

    fn update(&mut self, dt: f32) {
        self.camera_ctrl.update_camera(dt);
        self.camera_uniform = self.camera_ctrl.camera.view_projection().into();
        self.screen.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    #[allow(dead_code)]
    fn n_verts(&self) -> u32 {
        VERTICES.len() as u32
    }

    fn redraw(&mut self, ctrl_flow: &EventLoopWindowTarget<()>, dt: f32) {
        {
            if self.screen.config.width == 0 || self.screen.config.height == 0 {
                info!("ignoring input until window size has been configured");
                return;
            }
            // info!("redraw requested");
            self.update(dt);
            match self.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => self.screen.reset_size(),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    ctrl_flow.exit();
                    // control_flow.exit();
                }
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }

    fn n_inds(&self) -> u32 {
        INDICES.len() as u32
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.screen.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });
        let mut encoder =
            self.screen
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        render_pass.set_pipeline(&self.active_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(2, &self.iso_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.n_inds(), 0, 0..1); // 3.
        drop(render_pass);

        self.screen.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn init() -> (EventLoop<()>, Window) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    info!("hello world!");

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _size = window.request_inner_size(PhysicalSize::new(900, 600));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
    (event_loop, window)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    #[cfg(not(target_arch = "wasm32"))]
    use std::time::{Duration, Instant};
    #[cfg(target_arch = "wasm32")]
    use web_time::{Duration, Instant};

    let (mut event_loop, window) = init();

    let mut state = State::new(&window).await;
    let window = &window;
    let dt = 1.0 / 100.0; // secs
                          // event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, control_flow| {
            let event = match event {
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => event,
                Event::AboutToWait => {
                    let duration = Duration::from_millis((dt * 1000.0) as u64);
                    let alarm = Instant::now() + duration;
                    control_flow.set_control_flow(ControlFlow::WaitUntil(alarm));
                    state.redraw(control_flow, dt);

                    return;
                }
                _ => {
                    return;
                }
            };

            match event {
                WindowEvent::Resized(new_size) => {
                    info!("window resized to: {:#?}", new_size);
                    state.screen.resize(new_size)
                }
                WindowEvent::RedrawRequested => {
                    state.redraw(control_flow, dt);
                    // window.request_redraw();
                    // window.set_control_flow();
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    info!("Closing, bye...");
                    control_flow.exit()
                }
                event if state.input(&event) => {
                    dbg!(event);
                    dbg!(&state.camera_ctrl);
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .unwrap()
}
