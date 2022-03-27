use bevy::prelude::*;
use glam::Vec3;

pub struct Materials {
    pub sphere_material_red: Handle<StandardMaterial>,
    pub sphere_material_green: Handle<StandardMaterial>,
    pub sphere_material_blue: Handle<StandardMaterial>,
    pub plane_material : Handle<StandardMaterial>
}

pub struct Meshes {
    pub sphere_mesh: Handle<Mesh>,
    pub plane_mesh: Handle<Mesh>
}

pub struct BevyCamera {
    pub pos : Vec3,
    pub focal : Vec3
}

impl Default for BevyCamera {
    fn default() -> Self {
        BevyCamera{
            focal : Vec3::ZERO,
            pos : Vec3::new(-2.0, 2.5, 5.0),
        }
    }
}

#[derive(Clone, Copy, Component)]
pub struct BevyLight {
    pub pos : Vec3,
    pub intensity : f32
    // pub color : Color (illumination color)
    // pub radius : f32 (radius of the "sun ball")
    // pub range: f32 (max range of illumination)
}

impl Default for BevyLight {
    fn default() -> Self {
        BevyLight{
            pos: Vec3::new(4.0, 8.0, 4.0),
            intensity: 1400.0,
        }
    }
}

