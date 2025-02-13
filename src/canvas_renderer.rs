use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use stunts_engine::{
    camera::{Camera, CameraBinding},
    dot::RingDot,
    editor::{rgb_to_wgpu, ControlMode, Editor, Point, WebGpuResources, WindowSize, WindowSizeShader},
    vertex::Vertex,
};
use wasm_bindgen::prelude::Closure;
use web_sys::{window, HtmlCanvasElement};
use winit::{dpi::LogicalSize, event_loop, window::WindowBuilder};
use leptos::wasm_bindgen::JsCast;
use wgpu::{util::DeviceExt, StoreOp};

pub struct CanvasRenderer {
    pub editor: Arc<Mutex<Editor>>,
    pub gpu_resources: Arc<WebGpuResources>,
    pub render_pipeline: Arc<wgpu::RenderPipeline>,
    pub depth_view: Option<Arc<wgpu::TextureView>>,
    pub multisampled_view: Option<Arc<wgpu::TextureView>>
}

/// Call in this order:
/// new(editor)
/// recreate_depth_view(window_width, window_height)
/// begin_rendering()
impl CanvasRenderer {
    pub async fn new(editor_m: Arc<Mutex<Editor>>) -> CanvasRenderer {
        println!("Initializing Canvas Renderer...");

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("scene-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let height = canvas.height();
        let width = canvas.width();

        let window_size: WindowSize = WindowSize {
            width,
            height,
        };

        // gets surface, adapter, device, and queue
        let gpu_resources = WebGpuResources::request(canvas, window_size).await;

        let gpu_resources = Arc::new(gpu_resources);

        // let mut gpu_helper = GpuHelper::new();

        // let mut gpu_helper = Arc::new(gpu_helper);

        println!("Initializing pipeline...");

        let mut editor = editor_m.lock().unwrap();

        let camera = Camera::new(window_size);
        let camera_binding = CameraBinding::new(&gpu_resources.device);

        editor.camera = Some(camera);
        editor.camera_binding = Some(camera_binding);

        

        let sampler = gpu_resources
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

        // self.recreate_depth_view(
        //     &gpu_resources,
        //     window_size.width,
        //     window_size.height,
        // );

        let depth_stencil_state = wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };

        let camera_binding = editor
            .camera_binding
            .as_ref()
            .expect("Couldn't get camera binding");

        let model_bind_group_layout =
            gpu_resources
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        // Existing uniform buffer binding
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // Texture binding
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        // Sampler binding
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("model_bind_group_layout"),
                });

        let model_bind_group_layout = Arc::new(model_bind_group_layout);

        let group_bind_group_layout =
            gpu_resources
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        // Existing uniform buffer binding
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: Some("group_bind_group_layout"),
                });

        let group_bind_group_layout = Arc::new(group_bind_group_layout);

        let window_size_buffer =
            gpu_resources
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Window Size Buffer"),
                    contents: bytemuck::cast_slice(&[WindowSizeShader {
                        width: window_size.width as f32,
                        height: window_size.height as f32,
                    }]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let window_size_buffer = Arc::new(window_size_buffer);

        let window_size_bind_group_layout =
            gpu_resources
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
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
                });

        let window_size_bind_group_layout = Arc::new(window_size_bind_group_layout);

        let window_size_bind_group =
            gpu_resources
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &window_size_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: window_size_buffer.as_entire_binding(),
                    }],
                    label: None,
                });

        // Define the layouts
        let pipeline_layout =
            gpu_resources
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pipeline Layout"),
                    // bind_group_layouts: &[&bind_group_layout],
                    bind_group_layouts: &[
                        &camera_binding.bind_group_layout,
                        &model_bind_group_layout,
                        &window_size_bind_group_layout,
                        &group_bind_group_layout,
                    ], // No bind group layouts
                    push_constant_ranges: &[],
                });

        // Load the shaders
        let shader_module_vert_primary =
            gpu_resources
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Primary Vert Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("shaders/vert_primary.wgsl").into(),
                    ),
                });

        let shader_module_frag_primary =
            gpu_resources
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Primary Frag Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("shaders/frag_primary.wgsl").into(),
                    ),
                });

        // let swapchain_capabilities = gpu_resources
        //     .surface
        //     .get_capabilities(&gpu_resources.adapter);
        // let swapchain_format = swapchain_capabilities.formats[0]; // Choosing the first available format
        let swapchain_format = wgpu::TextureFormat::Bgra8Unorm; // hardcode for now - actually must match common-floem's

        // Configure the render pipeline
        let render_pipeline =
            gpu_resources
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Common Vector Primary Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    multiview: None,
                    cache: None,
                    vertex: wgpu::VertexState {
                        module: &shader_module_vert_primary,
                        entry_point: "vs_main", // name of the entry point in your vertex shader
                        buffers: &[Vertex::desc()], // Make sure your Vertex::desc() matches your vertex structure
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader_module_frag_primary,
                        entry_point: "fs_main", // name of the entry point in your fragment shader
                        targets: &[Some(wgpu::ColorTargetState {
                            format: swapchain_format,
                            // blend: Some(wgpu::BlendState::REPLACE),
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent {
                                    src_factor: wgpu::BlendFactor::SrcAlpha,
                                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                    operation: wgpu::BlendOperation::Add,
                                },
                                alpha: wgpu::BlendComponent {
                                    src_factor: wgpu::BlendFactor::One,
                                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                    operation: wgpu::BlendOperation::Add,
                                },
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    // primitive: wgpu::PrimitiveState::default(),
                    // depth_stencil: None,
                    // multisample: wgpu::MultisampleState::default(),
                    primitive: wgpu::PrimitiveState {
                        conservative: false,
                        topology: wgpu::PrimitiveTopology::TriangleList, // how vertices are assembled into geometric primitives
                        // strip_index_format: Some(wgpu::IndexFormat::Uint32),
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw, // Counter-clockwise is considered the front face
                        // none cull_mode
                        cull_mode: None,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Other properties such as conservative rasterization can be set here
                        unclipped_depth: false,
                    },
                    depth_stencil: Some(depth_stencil_state), // Optional, only if you are using depth testing
                    multisample: wgpu::MultisampleState {
                        count: 4, // effect performance
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                });

        let render_pipeline = Arc::new(render_pipeline);

        println!("Initialized...");

        let cursor_ring_dot = RingDot::new(
            &gpu_resources.device,
            &gpu_resources.queue,
            &model_bind_group_layout,
            &group_bind_group_layout,
            &window_size,
            Point { x: 600.0, y: 300.0 },
            rgb_to_wgpu(250, 20, 10, 255.0 / 2.0),
            &camera,
        );

        editor.cursor_dot = Some(cursor_ring_dot);

        // gpu_helper.gpu_resources = Some(Arc::clone(&gpu_resources));
        editor.gpu_resources = Some(Arc::clone(&gpu_resources));
        editor.model_bind_group_layout = Some(model_bind_group_layout);
        editor.group_bind_group_layout = Some(group_bind_group_layout);
        editor.window_size_bind_group = Some(window_size_bind_group);
        editor.window_size_bind_group_layout = Some(window_size_bind_group_layout);
        editor.window_size_buffer = Some(window_size_buffer);

        editor.update_camera_binding();

        drop(editor);

        Self {
            editor: editor_m,
            render_pipeline,
            gpu_resources,
            // gpu_helper
            depth_view: None,
            multisampled_view: None
        }
    }

    // call right after new() and before begin_rendering() also when changing window size
    pub fn recreate_depth_view(
        &mut self,
        // gpu_resources: &std::sync::Arc<WebGpuResources>,
        // window_size: &WindowSize,
        window_width: u32,
        window_height: u32,
    ) {
        let texture_format = wgpu::TextureFormat::Bgra8Unorm;

        // let config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: texture_format,
        //     width: window_width,
        //     height: window_height,
        //     present_mode: wgpu::PresentMode::Fifo,
        //     alpha_mode: wgpu::CompositeAlphaMode::Auto,
        //     // alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
        //     // alpha_mode: wgpu::CompositeAlphaMode::Inherit,
        //     view_formats: vec![],
        //     desired_maximum_frame_latency: 2,
        // };

        let surface = self.gpu_resources.surface.as_ref().expect("Couldn't get surface");
        let mut config = surface.get_default_config(&self.gpu_resources.adapter, window_width, window_height).unwrap();

        surface.configure(&self.gpu_resources.device, &config);

        let multisampled_texture = self.gpu_resources
            .device
            .create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: window_width,
                    height: window_height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 4,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                label: Some("Multisampled render texture"),
                view_formats: &[],
            });

        let multisampled_texture = Arc::new(multisampled_texture);

        let multisampled_view =
            multisampled_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let multisampled_view = Arc::new(multisampled_view);

        let depth_texture = self.gpu_resources
            .device
            .create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: window_width.clone(),
                    height: window_height.clone(),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 4, // used in a multisampled environment
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                label: Some("Depth Texture"),
                view_formats: &[],
            });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let depth_view = Arc::new(depth_view);

        self.depth_view = Some(depth_view);
        self.multisampled_view = Some(multisampled_view);

        // (depth_texture, depth_view)
    }

    pub fn begin_rendering(&self) {
        let editor = self.editor.clone();
        let gpu_resources = self.gpu_resources.clone();
        let render_pipeline = self.render_pipeline.clone();
        let depth_view = self.depth_view.as_ref().expect("Couldn't get depth view").clone();
        let multisampled_view = self.multisampled_view.as_ref().expect("Couldn't get depth view").clone();

        // web-based rendering loop
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let closure = Closure::wrap(Box::new(move || {
            // if !is_rendering_paused() {
                // let device = device.clone();
                // let state_guard = state.lock().unwrap();

                render_frame(
                    &editor,
                    &gpu_resources,
                    &render_pipeline,
                    &depth_view,
                    &multisampled_view
                    // &camera_bind_group,
                    // &camera_uniform_buffer,
                );

                // drop(state_guard);
            // }

            // Schedule the next frame
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>);

        *g.borrow_mut() = Some(closure);

        // Start the rendering loop
        request_animation_frame(g.borrow().as_ref().unwrap());



    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn render_frame(
    editor: &Arc<Mutex<Editor>>,
    // surface: &wgpu::Surface,
    // device: &wgpu::Device,
    // queue: &wgpu::Queue,
    gpu_resources: &Arc<WebGpuResources>,
    render_pipeline: &Arc<wgpu::RenderPipeline>,
    depth_view: &Arc<wgpu::TextureView>,
    multisampled_view: &Arc<wgpu::TextureView>,
    // camera_bind_group: &wgpu::BindGroup,
    // camera_uniform_buffer: &wgpu::Buffer,
) {
    let mut editor  = editor.lock().unwrap();

    let camera = editor.camera.expect("Couldn't get camera");

    let surface = &gpu_resources.surface.as_ref().expect("Couldn't get surface");
    let device = &gpu_resources.device;
    let queue = &gpu_resources.queue;

    // Render a frame
    let frame = surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };
        // let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //     label: Some("Render Pass"),
        //     color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        //         view: &view,
        //         resolve_target: None,
        //         ops: wgpu::Operations {
        //             load: wgpu::LoadOp::Clear(color),
        //             store: wgpu::StoreOp::Store,
        //         },
        //     })],
        //     // depth_stencil_attachment: None,
        //     depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        //         view: &depth_view, // This is the depth texture view
        //         depth_ops: Some(wgpu::Operations {
        //             load: wgpu::LoadOp::Clear(1.0), // Clear to max depth
        //             store: wgpu::StoreOp::Store,
        //         }),
        //         stencil_ops: None, // Set this if using stencil
        //     }),
        //     timestamp_writes: None,
        //     occlusion_query_set: None,
        // });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &multisampled_view,       // Use the multisampled view here
                resolve_target: Some(&view), // Resolve to the swapchain texture
                ops: wgpu::Operations {
                    // load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    // store: StoreOp::Store,
                    // load: wgpu::LoadOp::Load,
                    // store: wgpu::StoreOp::Store,
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Discard,
                },
            })],
            // depth_stencil_attachment: None,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // draw calls...
        render_pass.set_pipeline(&render_pipeline);

        // draw objects
        editor.step_video_animations(&camera, None);
        editor.step_motion_path_animations(&camera, None);

        let camera_binding = editor
            .camera_binding
            .as_ref()
            .expect("Couldn't get camera binding");

        render_pass.set_bind_group(0, &camera_binding.bind_group, &[]);
        render_pass.set_bind_group(
            2,
            editor
                .window_size_bind_group
                .as_ref()
                .expect("Couldn't get window size group"),
            &[],
        );

        // draw static (internal) polygons
        for (poly_index, polygon) in editor.static_polygons.iter().enumerate() {
            // uniform buffers are pricier, no reason to over-update when idle
            if let Some(dragging_id) = editor.dragging_path_handle {
                if dragging_id == polygon.id {
                    polygon
                        .transform
                        .update_uniform_buffer(&gpu_resources.queue, &camera.window_size);
                }
            }

            render_pass.set_bind_group(1, &polygon.bind_group, &[]);
            render_pass.set_bind_group(3, &polygon.group_bind_group, &[]);
            render_pass.set_vertex_buffer(0, polygon.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                polygon.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..polygon.indices.len() as u32, 0, 0..1);
        }

        // draw motion path static polygons, using motion path transform
        for (path_index, path) in editor.motion_paths.iter().enumerate() {
            // uniform buffers are pricier, no reason to over-update when idle
            if let Some(dragging_id) = editor.dragging_path {
                if dragging_id == path.id {
                    path.transform
                        .update_uniform_buffer(&gpu_resources.queue, &camera.window_size);
                }
            }

            render_pass.set_bind_group(3, &path.bind_group, &[]);

            for (poly_index, polygon) in path.static_polygons.iter().enumerate() {
                // uniform buffers are pricier, no reason to over-update when idle
                if let Some(dragging_id) = editor.dragging_path_handle {
                    if dragging_id == polygon.id {
                        polygon.transform.update_uniform_buffer(
                            &gpu_resources.queue,
                            &camera.window_size,
                        );
                    }
                }

                render_pass.set_bind_group(1, &polygon.bind_group, &[]);
                render_pass.set_vertex_buffer(0, polygon.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    polygon.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..polygon.indices.len() as u32, 0, 0..1);
            }
        }

        // draw polygons
        for (poly_index, polygon) in editor.polygons.iter().enumerate() {
            if !polygon.hidden {
                // uniform buffers are pricier, no reason to over-update when idle
                // also need to remember to update uniform buffers after changes like scale, rotation, position
                if let Some(dragging_id) = editor.dragging_polygon {
                    if dragging_id == polygon.id {
                        polygon.transform.update_uniform_buffer(
                            &gpu_resources.queue,
                            &camera.window_size,
                        );
                    }
                } else if editor.is_playing {
                    // still need to be careful of playback performance
                    polygon
                        .transform
                        .update_uniform_buffer(&gpu_resources.queue, &camera.window_size);
                }

                render_pass.set_bind_group(1, &polygon.bind_group, &[]);
                render_pass.set_bind_group(3, &polygon.group_bind_group, &[]);
                render_pass.set_vertex_buffer(0, polygon.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    polygon.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..polygon.indices.len() as u32, 0, 0..1);
            }
        }

        // draw text items
        for (text_index, text_item) in editor.text_items.iter().enumerate() {
            if !text_item.hidden {
                if !text_item.background_polygon.hidden {
                    // uniform buffers are pricier, no reason to over-update when idle
                    // also need to remember to update uniform buffers after changes like scale, rotation, position
                    if let Some(dragging_id) = editor.dragging_text {
                        if dragging_id == text_item.background_polygon.id {
                            text_item
                                .background_polygon
                                .transform
                                .update_uniform_buffer(
                                    &gpu_resources.queue,
                                    &camera.window_size,
                                );
                        }
                    } else if editor.is_playing {
                        // still need to be careful of playback performance
                        text_item
                            .background_polygon
                            .transform
                            .update_uniform_buffer(
                                &gpu_resources.queue,
                                &camera.window_size,
                            );
                    }

                    render_pass.set_bind_group(
                        1,
                        &text_item.background_polygon.bind_group,
                        &[],
                    );
                    render_pass.set_bind_group(
                        3,
                        &text_item.background_polygon.group_bind_group,
                        &[],
                    );
                    render_pass.set_vertex_buffer(
                        0,
                        text_item.background_polygon.vertex_buffer.slice(..),
                    );
                    render_pass.set_index_buffer(
                        text_item.background_polygon.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    render_pass.draw_indexed(
                        0..text_item.background_polygon.indices.len() as u32,
                        0,
                        0..1,
                    );
                }

                // uniform buffers are pricier, no reason to over-update when idle
                if let Some(dragging_id) = editor.dragging_text {
                    if dragging_id == text_item.id {
                        text_item.transform.update_uniform_buffer(
                            &gpu_resources.queue,
                            &camera.window_size,
                        );
                    }
                } else if editor.is_playing {
                    // still need to be careful of playback performance
                    text_item
                        .transform
                        .update_uniform_buffer(&gpu_resources.queue, &camera.window_size);
                }

                render_pass.set_bind_group(1, &text_item.bind_group, &[]);
                render_pass.set_bind_group(3, &text_item.group_bind_group, &[]);
                render_pass.set_vertex_buffer(0, text_item.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    text_item.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..text_item.indices.len() as u32, 0, 0..1);
            }
        }

        // draw image items
        for (image_index, st_image) in editor.image_items.iter().enumerate() {
            if !st_image.hidden {
                // uniform buffers are pricier, no reason to over-update when idle
                if let Some(dragging_id) = editor.dragging_image {
                    if dragging_id.to_string() == st_image.id {
                        st_image.transform.update_uniform_buffer(
                            &gpu_resources.queue,
                            &camera.window_size,
                        );
                    }
                } else if editor.is_playing {
                    // still need to be careful of playback performance
                    st_image
                        .transform
                        .update_uniform_buffer(&gpu_resources.queue, &camera.window_size);
                }

                render_pass.set_bind_group(1, &st_image.bind_group, &[]);
                render_pass.set_bind_group(3, &st_image.group_bind_group, &[]);
                render_pass.set_vertex_buffer(0, st_image.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    st_image.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..st_image.indices.len() as u32, 0, 0..1);
            }
        }

        // draw video items
        for (video_index, st_video) in editor.video_items.iter().enumerate() {
            if !st_video.hidden {
                // uniform buffers are pricier, no reason to over-update when idle
                if let Some(dragging_id) = editor.dragging_video {
                    if dragging_id.to_string() == st_video.id {
                        st_video.transform.update_uniform_buffer(
                            &gpu_resources.queue,
                            &camera.window_size,
                        );
                    }
                } else if editor.is_playing {
                    // still need to be careful of playback performance
                    st_video
                        .transform
                        .update_uniform_buffer(&gpu_resources.queue, &camera.window_size);
                }

                render_pass.set_bind_group(1, &st_video.bind_group, &[]);
                render_pass.set_bind_group(3, &st_video.group_bind_group, &[]);
                render_pass.set_vertex_buffer(0, st_video.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    st_video.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..st_video.indices.len() as u32, 0, 0..1);
            }
        }

        if let Some(dot) = &editor.cursor_dot {
            dot.transform
                .update_uniform_buffer(&gpu_resources.queue, &camera.window_size);
            render_pass.set_bind_group(1, &dot.bind_group, &[]);
            render_pass.set_bind_group(3, &dot.group_bind_group, &[]);
            render_pass.set_vertex_buffer(0, dot.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(dot.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..dot.indices.len() as u32, 0, 0..1);
        }

        // much more efficient than calling on mousemove??
        if editor.control_mode == ControlMode::Pan && editor.is_panning {
            editor.update_camera_binding();
        }
    }

    queue.submit(Some(encoder.finish()));
    device.poll(wgpu::Maintain::Poll);
    frame.present();
}