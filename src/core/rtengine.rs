use crate::utils::color::RGBColor;
use crate::utils::geometry::Point;
use crate::utils::geometry::Line;

const MAX_DEPTH: i32 = 10;

pub struct RTEngine{
    pub pos_camera : Point,
    pub pos_pixels : Vec<Point>,
}

impl RTEngine {
    pub fn run_engine(&mut self) -> Vec<RGBColor> {
        let mut colors = Vec::with_capacity(self.pos_pixels.len());

        for p in &self.pos_pixels {
            let ray = Line{
                p1 : self.pos_camera,
                p2 : *p
            };

            colors.push(color_contribution(ray, 0));
        }
        return colors
    }
}

fn color_contribution(ray : Line, depth : i32) -> RGBColor {
    let color = RGBColor { ..Default::default() }; //TODO : set background color

    if depth > MAX_DEPTH { return color };
    let p = intersect(ray);
    // if &p == std::ptr::null { return color};

    return color
}

fn intersect(ray : Line) -> Point{
    Point { ..Default::default() }
}

#[cfg(test)]
mod rtengine_tests{
    use crate::utils::color::RGBColor;
    use crate::utils::geometry::Point;
    use crate::core::rtengine::RTEngine;
    
    #[test]
    fn basic_instanciation() {
        let origin = Point{ ..Default::default() };
        let origins = Vec::from([origin, origin, origin]);

        let c = RGBColor{..Default::default()};
        let colors = Vec::from([c, c, c]);

        let rte = RTEngine{
            pos_camera : origin,
            pos_pixels : origins,
        };

        assert_eq!(rte.pos_camera, origin);
        assert_eq!(rte.pos_pixels.len(), 3);
    }
}