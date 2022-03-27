use crate::utils::geometry::Point;
use bevy::prelude::*;

pub struct Materials {
    pub sphere_material: Handle<StandardMaterial>,
}

pub struct Meshes {
    pub sphere_mesh: Handle<Mesh>,
}

pub fn sphere_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    meshes: Res<Meshes>,
    // pos: Point,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: materials.sphere_material.clone(),
        // transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
