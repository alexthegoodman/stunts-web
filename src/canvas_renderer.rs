use std::sync::{Arc, Mutex};

use stunts_engine::{
    camera::{Camera, CameraBinding},
    dot::RingDot,
    editor::{rgb_to_wgpu, Editor, Point, WebGpuResources, WindowSize, WindowSizeShader},
    vertex::Vertex,
};
use web_sys::HtmlCanvasElement;
use winit::{dpi::LogicalSize, event_loop, window::WindowBuilder};
use leptos::wasm_bindgen::JsCast;
use wgpu::util::DeviceExt;
pub struct GpuHelper {
    pub depth_view: Option<wgpu::TextureView>,
    pub gpu_resources: Option<std::sync::Arc<WebGpuResources>>,
}

impl GpuHelper {
    pub fn new() -> Self {
        GpuHelper {
            depth_view: None,
            gpu_resources: None,
        }
    }

    pub fn recreate_depth_view(
        &mut self,
        gpu_resources: &std::sync::Arc<WebGpuResources>,
        // window_size: &WindowSize,
        window_width: u32,
        window_height: u32,
    ) {
        let depth_texture = gpu_resources
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

        self.depth_view = Some(depth_view);

        // (depth_texture, depth_view)
    }
}

pub struct CanvasRenderer {}

impl CanvasRenderer {
    pub async fn new(editor: Arc<Mutex<Editor>>) -> CanvasRenderer {
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

        let mut gpu_helper = GpuHelper::new();

        println!("Initializing pipeline...");

        let mut editor = editor.lock().unwrap();

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

            gpu_helper.recreate_depth_view(
            &gpu_resources,
            window_size.width,
            window_size.height,
        );

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
        let swapchain_format = wgpu::TextureFormat::Bgra8UnormSrgb; // hardcode for now - actually must match common-floem's

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

        gpu_helper.gpu_resources = Some(Arc::clone(&gpu_resources));
        editor.gpu_resources = Some(Arc::clone(&gpu_resources));
        editor.model_bind_group_layout = Some(model_bind_group_layout);
        editor.group_bind_group_layout = Some(group_bind_group_layout);
        editor.window_size_bind_group = Some(window_size_bind_group);
        editor.window_size_bind_group_layout = Some(window_size_bind_group_layout);
        editor.window_size_buffer = Some(window_size_buffer);

        editor.update_camera_binding();

        Self {}
    }
}
