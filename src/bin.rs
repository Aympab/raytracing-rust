pub mod utils;
pub mod bevy_utils;

use bevy::prelude::*;
use bevy_utils::mesh::create_sphere;
use utils::geometry::Point;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Raytracing app".to_string(),
            width : 500.0,
            height : 300.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        });
        // .insert_bundle(PickableBundle::default()); // <- Makes the mesh pickable.
                                                   // cube

        ///3 spheres
        let c = Point{..Default::default()};
        commands.spawn_bundle(create_sphere(&c, meshes.copy(), materials));
        
        commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { ..Default::default() })),
            material: materials.add(Color::rgb(0.2, 0.4, 0.6).into()),
            transform: Transform::from_xyz(2.0, 0.6, -2.0),
            ..Default::default()
        });

        commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { ..Default::default() })),
            material: materials.add(Color::rgb(0.2, 0.4, 0.6).into()),
            transform: Transform::from_xyz(2.0, 0.6, -2.0),
            ..Default::default()
        });

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
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
        // .insert_bundle(PickingCameraBundle::default()); // <- Sets the camera to use for picking.
}