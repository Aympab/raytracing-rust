// pub mod geometry;
use std::ops::Add;

// #[derive(Default)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Default for Point {
    fn default() -> Self { Point { x: 0.0f32, y: 0.0f32, z: 0.0f32 }}
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return &self.x == &other.x
            && &self.y == &other.y
            && &self.z == &other.z;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}