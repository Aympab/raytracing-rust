use bevy::prelude::*;

pub struct Materials {
    pub sphere_material: Handle<StandardMaterial>,
}

pub struct Meshes {
    pub sphere_mesh: Handle<Mesh>,
}

pub fn sphere_spawn(mut commands: Commands, materials: Res<Materials>, meshes: Res<Meshes>) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: materials.sphere_material.clone(),
        transform: Transform::from_xyz(2.0, 0.6, -2.0),
        ..Default::default()
    });
}
