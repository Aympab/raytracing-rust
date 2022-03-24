use crate::utils::color::*;

#[derive(Default)]
pub struct Material {
    pub color : ColorComponents,
    pub reflexion : f32,
    pub absrobti : f32,
}