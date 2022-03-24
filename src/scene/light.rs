use crate::utils::geometry::Point;
use crate::utils::color::*;

#[derive(Default)]
pub struct Light {
    //TODO : Add Focal point and ConeAngle and add it this to the compute engine
    pub color : ColorComponents,
    pub pos : Point,
}

#[cfg(test)]
mod light_tests{
    use crate::scene::light::Light;
    use crate::utils::geometry::Point;

    #[test]
    fn default_instanciation() {
        let light = Light{..Default::default()};

        assert_eq!(light.pos, Point{..Default::default()});
    }
}