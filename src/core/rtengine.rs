use crate::utils::color::RGBColor;
use crate::utils::geometry::Line;
// use crate::utils::geometry::Point;

use glam::Vec3A;
use ndarray::Array2;

const MAX_DEPTH: i32 = 10;

pub struct RTEngine {
    pub pos_camera: Vec3A,
    pub pos_pixels: Array2<Vec3A>,
    // pub pos_pixels : Vec<Vec3A>,
}

impl RTEngine {
    ///The simplest ray tracing algorithm : path tracing
    pub fn path_tracing(&mut self) -> Array2<RGBColor> {
        let width: usize = self.pos_pixels.shape()[0];
        let hieght: usize = self.pos_pixels.shape()[1];

        let mut colors = Array2::<RGBColor>::default((width, hieght));

        for ((i, j), p) in self.pos_pixels.indexed_iter() {
            let ray = Line {
                p1: self.pos_camera,
                p2: *p,
            };

            colors[[i, j]] = color_contribution(ray, 0);
        }

        return colors;
    }
}

fn color_contribution(ray: Line, depth: i32) -> RGBColor {
    let color = RGBColor {
        ..Default::default()
    }; //TODO : set background color

    if depth > MAX_DEPTH {
        return color;
    };
    let _p = intersect(ray);
    // if &p == std::ptr::null { return color};
    //TODO:
    return color;
}

fn intersect(_ray: Line) -> Vec3A {
    //TODO :
    Vec3A::ZERO
}

fn sphere_intersect(center: Vec3A, radius: f32, ray_origin: Vec3A, ray_direction: Vec3A) -> f32 {
    let b = ray_direction.dot(ray_origin - center) * 2.;
    let c = (ray_origin - center).length_squared() - radius * radius;
    let delta = b * b - 4. * c;
    let mut result = -1.;
    if delta > 0. {
        let s = delta.sqrt();
        let t1 = (-b + s) * 0.5;
        let t2 = (-b - s) * 0.5;
        result = t1.min(t2);
    }
    result
}
