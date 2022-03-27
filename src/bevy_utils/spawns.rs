use bevy::prelude::*;
use crate::bevy_utils::resources::*;

use super::components::{BevyLightC, BevyCameraC, BevySphereC};

pub fn camera_spawn(
    mut commands: Commands,
){
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(BevyCameraC);
}

pub fn light_spawn(
    mut commands: Commands,
    // light : BevyLight
){
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            // intensity: light.intensity,
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    })
    .insert(BevyLightC);
    // .insert(LightIntensityC::default());
}

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
    })
    .insert(BevySphereC);

    //Blue sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: materials.sphere_material_blue.clone(),
        transform: Transform::from_xyz(1.0, 0.8, -1.0),
        ..Default::default()
    })
    .insert(BevySphereC);

    //Green sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: materials.sphere_material_green.clone(),
        transform: Transform::from_xyz(-1.0, 0.53, -0.6),
        ..Default::default()
    })
    .insert(BevySphereC);
}

pub fn plane_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    meshes: Res<Meshes>
){
        // plane
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.plane_mesh.clone(),
            material: materials.plane_material.clone(),
            ..Default::default()
        });
}