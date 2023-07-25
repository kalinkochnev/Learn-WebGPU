use crate::vertex::{Vertex};

pub fn triangle_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu:: RenderPipeline {
    // include_wgsl!("shader.wgsl") is available as a macro
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Solid triangle shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[ Vertex::desc() ], // Vertex buffers to pass in from program 
        },
        fragment: Some(wgpu::FragmentState { // fragment is necessary for storing color data to
                                             // the surface
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState { // indicate color outputs (use surface
                                                     // format for ease of copy)
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE), // replace old pixel data with new data
                write_mask: wgpu::ColorWrites::ALL, // write all colors RGBA
            })]
        }),
        // Indicates how to interpret vertices when converting to triangle
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, // every 3 vertices = 1 triangle
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw, // triangles facing
            cull_mode: Some(wgpu::Face::Back),
            // v-- anything else requires Features::NON_FILL_POLYGON
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false, // Req Features::DEPTH_CLIP_CONTROL
            conservative: false, // Req Features::CONSERVATIVE_RASTERIZATION
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState { // a type of anti-aliasing
            count: 1, // How many samples to use
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None, // how many layers render attachments can have
    });

    render_pipeline
}


pub fn rainbow_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu:: RenderPipeline {
    // include_wgsl!("shader.wgsl") is available as a macro
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Rainbow triangle shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("challenge_shader.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[ Vertex::desc() ], // Vertex buffers to pass in from program 
        },
        fragment: Some(wgpu::FragmentState { // fragment is necessary for storing color data to
                                             // the surface
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState { // indicate color outputs (use surface
                                                     // format for ease of copy)
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE), // replace old pixel data with new data
                write_mask: wgpu::ColorWrites::ALL, // write all colors RGBA
            })]
        }),
        // Indicates how to interpret vertices when converting to triangle
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, // every 3 vertices = 1 triangle
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw, // triangles facing
            cull_mode: Some(wgpu::Face::Back),
            // v-- anything else requires Features::NON_FILL_POLYGON
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false, // Req Features::DEPTH_CLIP_CONTROL
            conservative: false, // Req Features::CONSERVATIVE_RASTERIZATION
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState { // a type of anti-aliasing
            count: 1, // How many samples to use
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None, // how many layers render attachments can have
    });

    render_pipeline
}

