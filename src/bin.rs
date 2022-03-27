pub mod bevy_utils;
pub mod utils;

use bevy::prelude::*;
use bevy_utils::resources::BevyCamera;
use bevy_utils::spawns::*;
use bevy_utils::resources::Materials;
use bevy_utils::resources::Meshes;

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
        .add_startup_stage("sphere_spawn_stage", SystemStage::single(sphere_spawn))
        .add_startup_stage("camera_spawn_stage", SystemStage::single(camera_spawn))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //We add the ressources "Materials" type we made so bevy knows them
    commands.insert_resource(Materials {
        sphere_material: materials.add(Color::rgb(0.2, 0.4, 0.6).into()),
    });

    //Same for the meshing
    commands.insert_resource(Meshes {
        sphere_mesh: meshes.add(Mesh::from(shape::UVSphere {
            ..Default::default()
        })),
    });

    commands.insert_resource(BevyCamera{
        focal : Vec3::ZERO,
        pos : Vec3::new(-2.0, 2.5, 5.0),
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

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
}
