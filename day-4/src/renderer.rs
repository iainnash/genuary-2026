use nannou::prelude::*;
use nannou::wgpu;
use wgpu_types::SamplerBindingType;

use crate::shader::DIFFERENCE_SHADER;
use crate::video_capture::{SharedFrame, HEIGHT, WIDTH};

// No longer need frame buffer size as GStreamer handles averaging
const FRAME_SIZE: usize = (WIDTH * HEIGHT * 4) as usize;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

use std::time::{Instant, Duration};
use std::cell::{Cell, RefCell};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct VideoRenderer {
    pub shared_frame: SharedFrame,
    _gst_pipeline: gstreamer::Pipeline,
    mosaic_texture: wgpu::Texture,  // The persistent texture that accumulates partial updates
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    
    // CPU-side mosaic buffer with interior mutability
    mosaic_buffer: RefCell<Vec<u8>>,
    frame_counter: Cell<usize>,
    last_update: Cell<Instant>,
    rng: RefCell<StdRng>,
    
    // Mosaic parameters
    square_size: usize,
    squares_per_update: usize,
    update_interval: Duration,
}

impl VideoRenderer {
    pub fn new(window: &Window, shared_frame: SharedFrame, gst_pipeline: gstreamer::Pipeline) -> Self {
        let device = window.device();

        // Create mosaic texture (this is what we'll display with partial updates)
        let mosaic_texture = wgpu::TextureBuilder::new()
            .size([WIDTH, HEIGHT])
            .format(wgpu::TextureFormat::Rgba8Unorm)
            .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
            .sample_count(1)
            .build(device);

        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Video Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: None,
            border_color: None,
            anisotropy_clamp: 1,
        });

        // Create bind group layout (simplified to only current texture and sampler)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Video Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Create bind group (use mosaic texture for display)
        let mosaic_view = mosaic_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Video Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&mosaic_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Video Shader"),
            source: wgpu::ShaderSource::Wgsl(DIFFERENCE_SHADER.into()),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Video Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Video Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: window.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Initialize mosaic buffer
        let mosaic_buffer = RefCell::new(vec![0u8; FRAME_SIZE]);
        
        // Initialize random number generator with a fixed seed for reproducibility
        let seed = 42;
        let rng = RefCell::new(StdRng::seed_from_u64(seed));

        Self {
            shared_frame,
            _gst_pipeline: gst_pipeline,
            mosaic_texture,
            bind_group,
            pipeline,
            mosaic_buffer,
            frame_counter: Cell::new(0),
            last_update: Cell::new(Instant::now()),
            rng,
            square_size: 32,
            squares_per_update: 20,
            update_interval: Duration::from_millis(200),
        }
    }

    pub fn update(&self, _app: &App) {
        // Check if it's time to update the mosaic
        let now = Instant::now();
        let last_update = self.last_update.get();
        if now.duration_since(last_update) < self.update_interval {
            return;
        }
        self.last_update.set(now);
        
        // Increment frame counter
        let current_count = self.frame_counter.get();
        self.frame_counter.set(current_count + 1);
    }

    // Function to update random squares in the mosaic buffer
    fn update_random_squares(&self, source: &[u8]) {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let square_size = self.rng.borrow_mut().gen_range(6..=128);
        
        // Get mutable references to the mosaic buffer and RNG
        let mut mosaic_buffer = self.mosaic_buffer.borrow_mut();
        let mut rng = self.rng.borrow_mut();
        
        // Update random squares
        for _ in 0..self.squares_per_update {
            // Generate random position for the square
            let x = (rng.gen::<usize>() % (width - square_size)) & !0x1F; // Align to 32-pixel boundary
            let y = (rng.gen::<usize>() % (height - square_size)) & !0x1F; // Align to 32-pixel boundary
            
            // Copy the square from source to target
            for dy in 0..square_size {
                for dx in 0..square_size {
                    let pixel_idx = ((y + dy) * width + (x + dx)) * 4;
                    if pixel_idx + 3 < source.len() && pixel_idx + 3 < mosaic_buffer.len() {
                        mosaic_buffer[pixel_idx] = source[pixel_idx];         // R
                        mosaic_buffer[pixel_idx + 1] = source[pixel_idx + 1]; // G
                        mosaic_buffer[pixel_idx + 2] = source[pixel_idx + 2]; // B
                        mosaic_buffer[pixel_idx + 3] = source[pixel_idx + 3]; // A
                    }
                }
            }
        }
    }

    pub fn render(&self, app: &App, frame: Frame) {
        let window = app.main_window();
        let queue = window.queue();
        let device = window.device();
        
        // Track if we have a new frame to process
        let mut new_frame_processed = false;

        // Process current frame if available
        if let Some(current_rgba) = self.shared_frame.take_current() {
            if current_rgba.len() == FRAME_SIZE {
                new_frame_processed = true;
                
                // Copy random squares from current frame to mosaic buffer
                self.update_random_squares(&current_rgba);
                
                // Upload only the updated mosaic buffer to the GPU
                // Note: This still uploads the whole buffer, but only the squares
                // that were updated have changed
                queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture: &self.mosaic_texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &self.mosaic_buffer.borrow(),
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(WIDTH * 4),
                        rows_per_image: Some(HEIGHT),
                    },
                    wgpu::Extent3d {
                        width: WIDTH,
                        height: HEIGHT,
                        depth_or_array_layers: 1,
                    },
                );
            }
        }

        // Skip vertex calculation and rendering if no new frame was processed
        if !new_frame_processed {
            return;
        }

        // Calculate aspect ratio corrected vertices
        let win_rect = app.window_rect();
        let video_aspect = WIDTH as f32 / HEIGHT as f32;
        let win_aspect = win_rect.w() / win_rect.h();

        let (w, h) = if win_aspect > video_aspect {
            (video_aspect / win_aspect, 1.0)
        } else {
            (1.0, win_aspect / video_aspect)
        };

        let vertices = [
            Vertex {
                pos: [-w, -h],
                uv: [0.0, 1.0],
            },
            Vertex {
                pos: [w, -h],
                uv: [1.0, 1.0],
            },
            Vertex {
                pos: [-w, h],
                uv: [0.0, 0.0],
            },
            Vertex {
                pos: [w, h],
                uv: [1.0, 0.0],
            },
        ];

        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Render
        let mut encoder = frame.command_encoder();
        let texture_view = frame.texture_view();

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Video Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..4, 0..1);
        }
    }
}
