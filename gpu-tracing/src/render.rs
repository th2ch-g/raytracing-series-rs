use bytemuck::{Pod, Zeroable};
use crate::camera::{Camera, CameraUniforms};

pub struct PathTracer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    uniforms: Uniforms,
    uniforms_buffer: wgpu::Buffer,
    display_pipeline: wgpu::RenderPipeline,
    display_bind_groups: [wgpu::BindGroup; 2],
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Uniforms {
    camera: CameraUniforms,
    width: u32,
    height: u32,
    frame_count: u32,
    _pad: u32,
}

impl PathTracer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        width: u32,
        height: u32,
    ) -> Self {
        device.on_uncaptured_error(Box::new(|error| {
            panic!("Aborting due to an error: {}", error);
        }));

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            // source: wgpu::ShaderSource::Wgsl(
            //     std::borrow::Cow::Borrowed(
            //         include_str!("shaders.wgsl")
            //     )
            // ),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("shaders.wgsl").into()
            ),
        });
        let (display_pipeline, display_layout) = {
            let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba32Float,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
            });

            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("display"),
                layout: Some(
                    &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        bind_group_layouts: &[&bind_group_layout],
                        ..Default::default()
                    }),
                ),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Ccw,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    ..Default::default()
                },
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "display_vs",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "display_fs",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });
            (pipeline, bind_group_layout)
        };

        let uniforms = Uniforms {
            camera: CameraUniforms::zeroed(),
            width,
            height,
            frame_count: 0,
            _pad: 0,
        };
        let uniforms_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let radiance_samples = {
            let desc = wgpu::TextureDescriptor {
                label: Some("radiance samples"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
                view_formats: &[],
            };
            [device.create_texture(&desc), device.create_texture(&desc)]
        };

        let display_bind_groups = {
            let views = [
                radiance_samples[0].create_view(&wgpu::TextureViewDescriptor::default()),
                radiance_samples[1].create_view(&wgpu::TextureViewDescriptor::default()),
            ];

            [
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &display_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                                buffer: &uniforms_buffer,
                                offset: 0,
                                size: None,
                            }),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&views[0]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&views[1]),
                        },
                    ],
                }),

                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &display_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                                buffer: &uniforms_buffer,
                                offset: 0,
                                size: None,
                            }),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&views[1]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&views[0]),
                        },
                    ],
                }),
            ]
        };

        Self {
            device,
            queue,
            uniforms,
            uniforms_buffer,
            display_pipeline,
            display_bind_groups,
        }
    }

    pub fn reset_samples(&mut self) {
        self.uniforms.frame_count = 0;
    }

    pub fn render_frame(&mut self, camera: &Camera, target: &wgpu::TextureView) {
        self.uniforms.camera = *camera.uniforms();
        self.uniforms.frame_count += 1;
        self.queue.write_buffer(&self.uniforms_buffer, 0, bytemuck::bytes_of(&self.uniforms));

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render frame"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("display pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        render_pass.set_pipeline(&self.display_pipeline);
        render_pass.set_bind_group(
            0,
            &self.display_bind_groups[(self.uniforms.frame_count % 2) as usize],
            &[],
        );

        render_pass.draw(0..6, 0..1);

        drop(render_pass);

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
    }
}


