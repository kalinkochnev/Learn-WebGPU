use bytemuck;

use crate::linalg::SMatrix;

/// bytemuck::Pod means that the struct is "Plain Old Data" and is interpretable
/// as &[u8]
/// bytemuck::Zeroable means that struct attributes can be set to 0 (references
/// cannot be zeroed) [docs](https://doc.rust-lang.org/std/mem/fn.zeroed.html)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
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

    pub fn from_vector(vec: SMatrix<f32, 3, 1>, color: [f32; 3]) -> Vertex {
        Vertex {
            position: vec.col(0).unwrap(),
            color
        }
    }
}


pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

pub const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
