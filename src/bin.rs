pub mod bevy_utils;
pub mod utils;

use bevy::prelude::*;
use bevy_utils::mesh::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Raytracing app".to_string(),
            width: 500.0,
            height: 300.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_stage("sphere_spawning_stage", SystemStage::single(sphere_spawn))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Materials {
        sphere_material: materials.add(Color::rgb(0.2, 0.4, 0.6).into()),
    });

    commands.insert_resource(Meshes {
        sphere_mesh: meshes.add(Mesh::from(shape::UVSphere {
            ..Default::default()
        })),
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // .insert_bundle(PickableBundle::default()); // <- Makes the mesh pickable.

    //Light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    // .insert_bundle(PickingCameraBundle::default()); // <- Sets the camera to use for picking.
}
