use std::ops::Add;

#[derive(Debug, Copy, Clone)]
/// Point represents a basic 3D point in space
pub struct Point {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

/// By default, the position of a point is the origin : 0,0,0
impl Default for Point {
    fn default() -> Self { Point { x: 0.0f32, y: 0.0f32, z: 0.0f32 }}
}

/// Point == Point if x, y and z are equals
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return &self.x == &other.x
            && &self.y == &other.y
            && &self.z == &other.z;
    }
}

/// Adding two points is adding all of their components
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


#[cfg(test)]
mod geometry_tests{
    use crate::utils::geometry::Point;

    #[test]
    fn default_instanciation() {
        let p = Point{..Default::default()};

        assert_eq!(p.x, 0.0f32);
        assert_eq!(p.y, 0.0f32);
        assert_eq!(p.z, 0.0f32);
    }

    #[test]
    fn simple_add() {
        let offset = 10f32;

        let p1 = Point{..Default::default()};
        let p2 = Point{x : offset, y : offset, z : offset};

        let p = p1 + p2;

        assert_eq!(p.x, p1.x + p2.x);
        assert_eq!(p.y, p1.y + p2.y);
        assert_eq!(p.z, p1.z + p2.z);
    }

    #[test]
    fn simple_eq() {
        let v = 10f32;

        let p1 = Point{x : v, y : v, z : v};
        let p2 = Point{x : v, y : v, z : v};

        assert_eq!(p1, p2);
    }

    #[test]
    fn various_operator() {
        let mut p1 = Point{..Default::default()};
        let mut sum : f32 = 0f32;

        for i in 0..100 {
            let n = i as f32; 
            p1.x += n;
            p1.y += n;
            p1.z += n;

            sum += n;
        }

        let p2 = Point{ x : sum, y : sum, z : sum};
        assert_eq!(p1, p2);
        assert_eq!(p1 + p2, p1 + p1);
    }
}
