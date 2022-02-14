use bevy::prelude::*;

pub mod world;
use world::generate_chunk;



pub fn update_chunks(
    mut commands  : Commands,
    mut meshes    : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>
) -> () {
    let mesh = generate_chunk([0, 0, 0]);
    commands.spawn_bundle(PbrBundle {
        mesh      : meshes.add(mesh),
        material  : materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform : Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}
