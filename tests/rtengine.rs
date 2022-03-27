#[cfg(test)]
mod rtengine_tests {
    use ndarray::Array2;
    use raytracing::core::rtengine::RTEngine;
    use raytracing::utils::geometry::Point;

    #[test]
    fn basic_instanciation() {
        let origin = Point {
            ..Default::default()
        };
        let origins = Array2::<Point>::default((3, 1));

        // let c = RGBColor{..Default::default()};
        // let colors = Vec::from([c, c, c]);

        let rte = RTEngine {
            pos_camera: origin,
            pos_pixels: origins,
        };

        assert_eq!(rte.pos_camera, origin);
        assert_eq!(rte.pos_pixels.len(), 3);
    }

    #[test]
    /// Size of array of pixels should be the same as the size
    /// of pos_pixels inputs after computing the path tracing algorithm.
    ///
    /// Meaning that if you have a screen of x*y pixels, you have x*y colors.
    /// pt = Path tracing
    fn pt_size_of_output() {
        let size: usize = 10;
        let pos_pixels = Array2::<Point>::default((size, size * 2));

        let mut rte = RTEngine {
            pos_camera: Point {
                ..Default::default()
            },
            pos_pixels: pos_pixels,
        };

        let c = rte.path_tracing();

        assert_eq!(c.len(), rte.pos_pixels.len());
    }
}
