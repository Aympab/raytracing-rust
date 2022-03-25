#[cfg(test)]
mod camera_tests{
    use raytracing::scene::camera::Camera;

    #[test]
    fn default_instanciation() {
        let cam = Camera{..Default::default()};

        assert_eq!(cam.pos, cam.pos_focal);
    }
}

#[cfg(test)]
mod light_tests{
    use raytracing::scene::light::Light;
    use raytracing::utils::geometry::Point;

    #[test]
    fn default_instanciation() {
        let light = Light{..Default::default()};

        assert_eq!(light.pos, Point{..Default::default()});
    }
}