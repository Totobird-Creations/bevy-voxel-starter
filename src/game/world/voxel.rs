use block_mesh;



#[derive(Clone, Debug)]
pub struct Colour(f32, f32, f32, f32);
impl Colour {
    pub fn r(&self) -> f32 {
        return self.0;
    }
    pub fn g(&self) -> f32 {
        return self.1;
    }
    pub fn b(&self) -> f32 {
        return self.2;
    }
    pub fn a(&self) -> f32 {
        return self.3;
    }
}

impl PartialEq for Colour {
    fn eq(&self, other: &Self) -> bool {
        return (self.r() == other.r()) && (self.g() == other.g()) && (self.b() == other.b()) && (self.a() == other.a());
    }
}
impl Eq for Colour {}



#[derive(Clone, Debug)]
pub enum Model {
    Cube(
        Colour /* Colour of cube */
    ),
    Model(
        String /* Path to model  */,
        bool   /* Is opaque      */
    ),
    None       /* Empty model    */
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        match (self) {
            Model::Cube(colour_a) => {
                match (other) {
                    Model::Cube(colour_b) => return colour_a == colour_b,
                    _                     => return false
                }
            },
            Model::Model(path_a, opaque_a) => {
                match (other) {
                    Model::Model(path_b, opaque_b) => return (path_a == path_b) && (opaque_a == opaque_b),
                    _                              => false
                }
            },
            Model::None => {
                match (other) {
                    Model::None => return true,
                    _           => return false
                }
            }
        }
    }
}
impl Eq for Model {}



#[derive(Clone, Debug)]
pub struct Voxel(Model);

impl PartialEq for Voxel {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}
impl Eq for Voxel {}

impl block_mesh::Voxel for Voxel {

    fn is_empty(&self) -> bool {
        match (&self.0) {
            Model::Cube(_color)          => false,
            Model::Model(_path, _opaque) => false,
            Model::None                  => true
        }
    }

    fn is_opaque(&self) -> bool {
        match (&self.0) {
            Model::Cube(color)          => color.a() == 1.0,
            Model::Model(_path, opaque) => *opaque,
            Model::None                 => false
        }
    }

}

impl block_mesh::MergeVoxel for Voxel {

    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        return self.clone();
    }

}



pub mod voxels {
    use super::{
        Colour,
        Model,
        Voxel
    };

    pub const EMPTY : Voxel = Voxel(Model::None);
    pub const SOLID : Voxel = Voxel(Model::Cube(Colour(1.0, 1.0, 1.0, 1.0)));

}
