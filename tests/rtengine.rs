#[cfg(test)]
mod rtengine_tests{
    use raytracing::utils::geometry::Point;
    use raytracing::core::rtengine::RTEngine;
    
    #[test]
    fn basic_instanciation() {
        let origin = Point{ ..Default::default() };
        let origins = Vec::from([origin, origin, origin]);

        // let c = RGBColor{..Default::default()};
        // let colors = Vec::from([c, c, c]);

        let rte = RTEngine{
            pos_camera : origin,
            pos_pixels : origins,
        };

        assert_eq!(rte.pos_camera, origin);
        assert_eq!(rte.pos_pixels.len(), 3);
    }
}