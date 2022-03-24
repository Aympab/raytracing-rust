use crate::utils::geometry::Point;

#[derive(Default)]
pub struct Camera {
    pub pos : Point,
    pub pos_focal : Point,
}

#[cfg(test)]
mod camera_tests{
    use crate::scene::camera::Camera;

    #[test]
    fn default_instanciation() {
        let cam = Camera{..Default::default()};

        assert_eq!(cam.pos, cam.pos_focal);
    }
}