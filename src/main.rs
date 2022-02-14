#![allow(unused_parens)]

use bevy::{
    prelude::*,
    input::mouse::MouseMotion,
    pbr::wireframe::{WireframePlugin, WireframeConfig}
};
// Remove before release.
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

pub mod game;



fn main() -> () {
    let mut app = App::new();

    // Setup environment and window.
    app.insert_resource(Msaa {samples : 4});
    app.insert_resource(WindowDescriptor {
        title  : String::from("Voxel Test"),
        width  : 1024.0,
        height : 600.0,
        ..Default::default()
    });

    // Add default plugin.
    app.add_plugins(DefaultPlugins);
    app.add_plugin(WireframePlugin);
    // Remove before release.
    app.add_plugin(FlyCameraPlugin);

    // Setup systems.
    app.add_startup_system(setup);
    app.add_startup_system(game::update_chunks);

    app.run();
}



fn setup(
    mut commands         : Commands,
    mut meshes           : ResMut<Assets<Mesh>>,
    mut materials        : ResMut<Assets<StandardMaterial>>,
    mut wireframe_config : ResMut<WireframeConfig>,
    mut windows          : ResMut<Windows>
) -> () {
    wireframe_config.global = true;

    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);

    commands.spawn_bundle(PbrBundle {
        mesh      : meshes.add(Mesh::from(shape::Plane {size : 1.0})),
        material  : materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform : Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle::default()).insert(FlyCamera::default());
    commands.spawn_bundle(PointLightBundle {
        transform : Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}