use bytemuck;

use crate::linalg::SMatrix;

/// bytemuck::Pod means that the struct is "Plain Old Data" and is interpretable
/// as &[u8]
/// bytemuck::Zeroable means that struct attributes can be set to 0 (references
/// cannot be zeroed) [docs](https://doc.rust-lang.org/std/mem/fn.zeroed.html)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    // In order for the macro to retain the value of its output, need to store in
    // const
    const ATTRIBS: [wgpu:: VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    /// Creates a description of the memory layout of the Vertex struct
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // Width of a single vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS
        // This is the verbose method of specifying the vertex buffer layout
        //     attributes: &[
        //         wgpu::VertexAttribute {
        //             offset: 0, // offset from start of vertex struct
        //             shader_location: 0, // used by wgsl to populate its struct
        //             format: wgpu::VertexFormat::Float32x3, // essentially
        //         },
        //         wgpu::VertexAttribute {
        //             offset: std::mem::sizeof::<[f32, 3]>() as wgpu::BufferAddress,
        //             shader_location: 1,
        //             format: wgpu::VertexFormat::Float32x3
        //         }
        //     ]
        }
    }

    pub fn from_vector(vec: SMatrix<f32, 3, 1>, color: &[f32; 3]) -> Vertex {
        Vertex {
            position: vec.col(0).unwrap(),
            color: *color
        }
    }
}
