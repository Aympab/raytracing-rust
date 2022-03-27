use bevy::prelude::*;
use glam::Vec3;

pub struct Materials {
    pub sphere_material: Handle<StandardMaterial>,
}

pub struct Meshes {
    pub sphere_mesh: Handle<Mesh>,
}

pub struct BevyCamera {
    pub pos : Vec3,
    pub focal : Vec3
}