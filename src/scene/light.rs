use crate::utils::geometry::Point;
use crate::utils::color::*;

#[derive(Default)]
pub struct Light {
    //TODO : Add Focal point and ConeAngle and add it this to the compute engine
    pub color : ColorComponents,
    pub pos : Point,
}