use crate::utils::geometry::Point;
use bevy::prelude::*;

pub fn create_sphere(
            center : &Point,
            meshes: ResMut<Assets<Mesh>>,
            materials: ResMut<Assets<StandardMaterial>>) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { ..Default::default() })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(center.x, center.y, center.z),
        ..Default::default()
    }
}