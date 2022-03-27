#[cfg(test)]
mod camera_tests {
    use raytracing::scene::camera::Camera;

    #[test]
    fn default_instanciation() {
        let cam = Camera {
            ..Default::default()
        };

        assert_eq!(cam.pos, cam.pos_focal);
    }
}

#[cfg(test)]
mod light_tests {
    use glam::Vec3A;
    use raytracing::scene::light::Light;

    #[test]
    fn default_instanciation() {
        let light = Light::default();

        assert_eq!(light.pos, Vec3A::ZERO);
    }
}
