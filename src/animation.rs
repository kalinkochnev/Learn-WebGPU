use crate::{vertex::Vertex, linalg::SMatrix, transform::Transform};

pub struct AnimationState {
   pub vertices: Vec<Vertex>,
   pub indices: Vec<u16>
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

impl AnimationState {
    pub fn new() -> Self {
       AnimationState {
           vertices: VERTICES.into(),
           indices: INDICES.into(), 
       } 
    }
 
    pub fn rotating_pentagon(&mut self, del_time: u128) {
        let velocity: f32 = 0.55; // rad per sec 
        for vert in self.vertices.iter_mut() {
            let new_pos = SMatrix::rotation(velocity * (del_time as f32 / 10f32.powf(9.0)) as f32) * SMatrix::vector(vert.position);
            vert.position = new_pos.col(0).unwrap();
        }
        println!("{:?}", self.vertices[0]);
    }
}
