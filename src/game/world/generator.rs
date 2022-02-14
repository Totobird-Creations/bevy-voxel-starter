use bevy::{
    prelude::Mesh,
    render::{
        mesh::{
            Indices,
            VertexAttributeValues
        },
        render_resource::PrimitiveTopology
    }
};
use block_mesh::{
    ndshape::ConstShape,
    GreedyQuadsBuffer,
    RIGHT_HANDED_Y_UP_CONFIG,
    greedy_quads
};
use super::{
    ChunkSize,
    voxel::{
        Voxel,
        voxels
    }
};

mod flat;
pub use flat::FlatGenerator;



pub struct ChunkArray<'l> {
    pub array : [[[&'l Voxel; ChunkSize::ARRAY[2] as usize]; ChunkSize::ARRAY[1] as usize]; ChunkSize::ARRAY[0] as usize]
}
impl<'l> ChunkArray<'l> {

    pub fn new() -> ChunkArray<'l> {
        let array = [[[&voxels::EMPTY; ChunkSize::ARRAY[2] as usize]; ChunkSize::ARRAY[1] as usize]; ChunkSize::ARRAY[0] as usize];
        return ChunkArray {
            array : array
        };
    }

    pub fn push(&mut self, position : [u32; 3], voxel : &'l Voxel) -> () {
        let size = self.get_size();
        if (position[0] >= size[0]) {
            panic!("X position {} is out of range 0..{}.", position[0], size[0] - 1)
        } else if (position[1] >= size[1]) {
            panic!("Y position {} is out of range 0..{}.", position[1], size[1] - 1)
        } else if (position[2] >= size[2]) {
            panic!("Z position {} is out of range 0..{}.", position[2], size[2] - 1)
        }
        self.array[position[0] as usize][position[1] as usize][position[2] as usize] = &voxel;
    }

    pub fn get_size(&self) -> [u32; 3] {
        return ChunkSize::ARRAY;
    }

    pub fn as_mesh(&self) -> Mesh {

        let mut samples = [voxels::EMPTY; ChunkSize::SIZE as usize];
        for i in 0..ChunkSize::SIZE {
            let [x, y, z] = ChunkSize::delinearize(i);
            samples[i as usize] = self.array[x as usize][y as usize][z as usize].clone();
        }

        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

        let mut buffer = GreedyQuadsBuffer::new(samples.len());
        greedy_quads(
            &samples,
            &ChunkSize {},
            [0; 3],
            ChunkSize::ARRAY,
            &faces,
            &mut buffer
        );

        let     num_indices  = buffer.quads.num_quads() * 6;
        let     num_vertices = buffer.quads.num_quads() * 4;
        let mut indices      = Vec::with_capacity(num_indices);
        let mut positions    = Vec::with_capacity(num_vertices);
        let mut normals      = Vec::with_capacity(num_vertices);
    
        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                indices   .extend_from_slice( &face.quad_mesh_indices   (positions.len() as u32) );
                positions .extend_from_slice( &face.quad_mesh_positions (&quad, 1.0)             );
                normals   .extend_from_slice( &face.quad_mesh_normals   ()                       );
            }
        }

        let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        render_mesh.set_attribute(
            "Vertex_Position",
            VertexAttributeValues::Float32x3(positions),
        );
        render_mesh.set_attribute("Vertex_Normal", VertexAttributeValues::Float32x3(normals));
        render_mesh.set_attribute(
            "Vertex_Uv",
            VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices]),
        );
        render_mesh.set_indices(Some(Indices::U32(indices.clone())));

        return render_mesh;

    }

}



pub trait Generator {

    fn generate_chunk(&self, chunk : [i32; 3], origin : [i32; 3]) -> ChunkArray;

}
