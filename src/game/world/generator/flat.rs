use super::{
    ChunkArray,
    Generator,
    voxels
};



pub struct FlatGenerator;

impl Generator for FlatGenerator {

    fn generate_chunk(&self, chunk : [i32; 3], _origin : [i32; 3]) -> ChunkArray {
        let mut array = ChunkArray::new();
        let     size  = array.get_size().clone();
        if (true) {
            for ix in 0..size[0] {
                for iy in 0..size[1] {
                    for iz in 0..size[2] {
                        let position_in_chunk = [ix, iy, iz];
                        array.push(position_in_chunk, &voxels::SOLID);
                    }
                }
            }
        }
        return array;
    }

}
