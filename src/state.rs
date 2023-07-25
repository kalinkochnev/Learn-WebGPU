use std::collections::HashMap;

use wgpu::util::DeviceExt;
use winit::{window::Window, event::WindowEvent};
use crate::vertex::{VERTICES, INDICES};
use crate::pipelines;

#[derive(PartialEq, Eq, Hash)]
pub enum PipelineMode {
    Solid,
    Rainbow,
}

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub mode: PipelineMode,
    color: wgpu::Color,
    pipelines: HashMap<PipelineMode, wgpu::RenderPipeline>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pub window: Window,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();
        assert!(size.height != 0, "Window height can't be 0");
        assert!(size.width != 0, "Window width can't be 0");

        // The instance is a handle to the gpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // The surface needs to live as long as the window that makes it
        // State owns the window and the surface

        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        // Shader code assumed sRGB format
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        // Initialize the vertex buffer
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX
            }
            );

        // Initialize the index buffer
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        let num_indices = INDICES.len() as u32;

        // Initialize the piplelines that can be switched between
        let mut pipelines = HashMap::<PipelineMode, wgpu::RenderPipeline>::new();
        pipelines.insert(PipelineMode::Solid, pipelines::triangle_pipeline(&device, &config));
        pipelines.insert(PipelineMode::Rainbow, pipelines::rainbow_pipeline(&device, &config));

        Self {
            window,
            size,
            config,
            surface,
            device,
            queue,
            vertex_buffer,
            index_buffer,
            num_indices,
            mode: PipelineMode::Solid,
            pipelines,
            color,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Returns a boolean to indicate an event has been fully processed
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.color = wgpu::Color {
                    r: position.x as f64 / self.size.width as f64,
                    g: position.y as f64 / self.size.height as f64,
                    b: 1.0,
                    a: 1.0
                };
                true
            },
            WindowEvent::KeyboardInput { input, .. } => {
                // If space is pressed (scancode = 52 for linux), enable use_color
                if input.scancode == 57 {
                    println!("Switched!");
                    match self.mode {
                        PipelineMode::Rainbow => self.mode = PipelineMode::Solid,
                        PipelineMode::Solid => self.mode = PipelineMode::Rainbow,
                    }
                    return true;
                }
                return false;
            }
            _ => false
        }
    }

    pub fn update(&mut self) {
        // TODO
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // provides new SurfaceTexture we will use to render to
        let output = self.surface.get_current_texture()?;

        // We can control how the texture will interact with the render code
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // GPU expects commands to be stored into buffer before sending
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // begin_render_pass() borrows `encoder` mutably. encoder.finish()
        // cannot be called until the borrow ends
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what is targeted by frament shader using @location(0) in shader.wgsl
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.color),
                            store: true,
                        },
                    })
                ],
                depth_stencil_attachment: None,
            });

            self.set_pipelines(&mut render_pass);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn set_pipelines<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(self.pipelines.get(&self.mode).unwrap());
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

        // Draw something with N vertices and 1 instance
        // In WGSL, this is stored as @builtin(vertex_index)
    }
}
