use bevy::prelude::*;
use crate::bevy_utils::resources::*;

pub fn sphere_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    meshes: Res<Meshes>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: materials.sphere_material.clone(),
        // transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
