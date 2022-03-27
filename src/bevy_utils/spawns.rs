use bevy::prelude::*;
use crate::bevy_utils::resources::*;

pub fn sphere_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    meshes: Res<Meshes>,
) {

    //Red sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: materials.sphere_material_red.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    //Blue sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: materials.sphere_material_blue.clone(),
        transform: Transform::from_xyz(1.0, 0.8, -1.0),
        ..Default::default()
    });

    //Green sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: materials.sphere_material_green.clone(),
        transform: Transform::from_xyz(-1.0, 0.53, -0.6),
        ..Default::default()
    });
}

pub fn camera_spawn(
    mut commands: Commands,
    cam: Res<BevyCamera>,
){
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(cam.pos).looking_at(cam.focal, Vec3::Y),
        ..Default::default()
    });
}
