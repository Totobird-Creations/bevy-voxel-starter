use super::{
    ChunkArray,
    Generator,
    voxels
};



pub struct FlatGenerator;

impl Generator for FlatGenerator {

    fn generate_chunk(&self, _chunk : [i32; 3], _origin : [i32; 3]) -> ChunkArray {
        let mut array = ChunkArray::new();
        let     size  = array.get_size().clone();
        for ix in 0..size[0] {
            for iz in 0..size[2] {
                array.push([ix, 0, iz], &voxels::SOLID);
                if (ix == iz) {
                    array.push([ix, 1, iz], &voxels::SOLID);
                }
            }
        }
        return array;
    }

}
