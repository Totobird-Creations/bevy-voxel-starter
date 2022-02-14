#![allow(unused_parens)]

use bevy::{
    prelude::*,
    input::mouse::MouseMotion
};

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

    // Setup systems.
    app.add_startup_system(setup);
    app.add_startup_system(game::update_chunks);

    app.run();
}



fn setup(
    mut commands  : Commands,
    mut meshes    : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
    mut windows   : ResMut<Windows>
) -> () {
    /*let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);*/

    commands.spawn_bundle(PbrBundle {
        mesh      : meshes.add(Mesh::from(shape::Plane {size : 1.0})),
        material  : materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform : Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform : Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0)
        )),
        ..Default::default()
    }).with(CameraRotator);
    commands.spawn_bundle(PointLightBundle {
        transform : Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}



#[derive(Component)]
struct CameraRotator {
    yaw   : f32,
    pitch : f32
}

fn camera(
    time                    : Res<Time>,
    mut mouse_motion_events : EventReader<MouseMotion>,
    mut query_camera        : Query<(With<Camera>, &mut Transform)>
) -> () {
    let (_, mut transform) = query_camera.single_mut();
    let mut delta : Vec2 = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }
    if (! delta.is_nan()) {
        let rotation = transform.rotation.xyz();
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, rotation.y + delta.y * time.delta_seconds() * 100.0);
    }
}