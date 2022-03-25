use crate::utils::color::RGBColor;
use crate::utils::geometry::Point;
use crate::utils::geometry::Line;

const MAX_DEPTH: i32 = 10;

pub struct RTEngine{
    pub pos_camera : Point,
    pub pos_pixels : Vec<Point>,
}

impl RTEngine {
    ///The simplest ray tracing algorithm : path tracing
    pub fn path_tracing(&mut self) -> Vec<RGBColor> {
        let mut colors = Vec::with_capacity(self.pos_pixels.len());

        for p in &self.pos_pixels {
            let ray = Line{
                p1 : self.pos_camera,
                p2 : *p
            };
            println!("Pushing mamene");
            colors.push(color_contribution(ray, 0));
        }
        return colors
    }
}

fn color_contribution(ray : Line, depth : i32) -> RGBColor {
    let color = RGBColor { ..Default::default() }; //TODO : set background color

    if depth > MAX_DEPTH { return color };
    let _p = intersect(ray);
    // if &p == std::ptr::null { return color};

    return color
}

fn intersect(_ray : Line) -> Point{
    Point { ..Default::default() }
}