use bevy::prelude::*;
use block_mesh::ndshape::ConstShape;

pub mod world;
use world::generate_chunk;



type ChunkSize = block_mesh::ndshape::ConstShape3u32<18, 18, 18>;



pub fn update_chunks(
    mut commands  : Commands,
    mut meshes    : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>
) -> () {
    for x in -1..2 {
        for z in -1..2 {
            spawn_chunk(&mut commands, &mut meshes, &mut materials, [x, 0, z]);
        }
    }
}



pub fn spawn_chunk(
    commands  : &mut Commands,
    meshes    : &mut Assets<Mesh>,
    materials : &mut Assets<StandardMaterial>,
    position  : [i32; 3]
) -> () {
    let size = ChunkSize::ARRAY;
    let mesh = generate_chunk(position);
    commands.spawn_bundle(PbrBundle {
        mesh     : meshes.add(mesh),
        material : materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform : Transform::from_translation(Vec3::new(
            (size[0] as f32) * (position[0] as f32),
            (size[1] as f32) * (position[1] as f32),
            (size[2] as f32) * (position[2] as f32)
        )),
        ..Default::default()
    });
}
