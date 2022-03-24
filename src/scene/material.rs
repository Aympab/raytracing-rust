use crate::utils::color::*;

#[derive(Default)]
pub struct Material {
    pub color : ColorComponents,
    pub reflexion : f32,
    pub refraction : f32,
    pub absorbtion : f32,
}