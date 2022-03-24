use crate::utils::geometry::Point;

pub struct RTEngine{
    pub p_camera : Point,
    pub p_pixels : Vec<Point>
}

impl RTEngine {
    pub fn run_engine(&mut self) {
        println!("Running engine");
    }
}

#[cfg(test)]
mod rtengine_tests{
    use crate::utils::geometry::Point;
    use crate::core::rtengine::RTEngine;
    
    #[test]
    fn basic_instanciation() {
        let origin = Point{ ..Default::default() };

        let origins = Vec::from([origin, origin, origin]);

        let rte = RTEngine{
            p_camera : origin,
            p_pixels : origins
        };

        assert_eq!(rte.p_camera, origin);
        assert_eq!(rte.p_pixels.len(), 3);
    }
}