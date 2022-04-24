use bevy::prelude::*;
use crate::bevy_utils::resources::*;
use bevy_mod_raycast::{RayCastSource, RayCastMesh};

/*
Mark our generic `RayCastMesh`s and `RayCastSource`s as part
of the same "RayCastSet". This plugin uses generics
to distinguish between groups of raycasters.
*/
pub struct MyRaycastSet;

use super::components::{BevyLightC, BevyCameraC, BevySphereC};

pub fn camera_spawn(
    mut commands: Commands,
){
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(RayCastSource::<MyRaycastSet>::new_transform_empty())
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

    let mut spawn_spheres = |pos : Vec3, mat : Handle<StandardMaterial>| {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.sphere_mesh.clone(),
        material: mat, //materials.sphere_material_red.clone(),
        transform: Transform::from_translation(pos),
        ..Default::default()
    })
    .insert(RayCastMesh::<MyRaycastSet>::default()) //make the mesh ray castable
    .insert(BevySphereC);
    };

    //Red sphere
    spawn_spheres(Vec3::new(0.0, 0.0, 0.0), materials.sphere_material_red.clone());
    
    //Blue sphere
    spawn_spheres(Vec3::new(1.0, 0.8, -1.0), materials.sphere_material_blue.clone());
    
    //Green sphere
    spawn_spheres(Vec3::new(-1.0, 0.53, -0.6), materials.sphere_material_green.clone());
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