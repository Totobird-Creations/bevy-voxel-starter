use bevy::prelude::Mesh;
use block_mesh::ndshape::ConstShape;

use super::ChunkSize;
pub mod generator;
use generator::{
    Generator,
    FlatGenerator
};
pub mod voxel;



pub fn generate_chunk(position : [i32; 3]) -> Mesh {

    let generator = FlatGenerator {};

    let size        = ChunkSize::ARRAY;
    let chunk_array = generator.generate_chunk(position, [
        position[0] * (size[0] as i32),
        position[1] * (size[1] as i32),
        position[2] * (size[2] as i32)
    ]);
    let mesh        = chunk_array.as_mesh();
    return mesh;

}
