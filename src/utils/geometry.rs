// use std::ops::Add;
use glam::Vec3A;

#[derive(Debug, Copy, Clone, Default)]
pub struct Line {
    pub p1 : Vec3A,
    pub p2 : Vec3A
}

// #[derive(Debug, Copy, Clone)]
// /// Point represents a basic 3D point in space
// pub struct Point {
//     pub x : f32,
//     pub y : f32,
//     pub z : f32,
// }

// /// By default, the position of a point is the origin : 0,0,0
// impl Default for Point {
//     fn default() -> Self { Point { x: 0.0f32, y: 0.0f32, z: 0.0f32 }}
// }

// impl Point{
//     pub fn xyz(x : f32, y : f32, z : f32) -> Self{
//         Point{ x : x, y : y, z : z}
//     }
// }

// /// Point == Point if x, y and z are equals
// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         return &self.x == &other.x
//             && &self.y == &other.y
//             && &self.z == &other.z;
//     }
// }

// /// Adding two points is adding all of their components
// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z
//         }
//     }
// }