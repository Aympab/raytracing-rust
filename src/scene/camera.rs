use crate::utils::geometry::Point;

#[derive(Default)]
pub struct Camera {
    pub pos : Point,
    pub pos_focal : Point,
}