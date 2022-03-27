pub mod core;
pub mod utils;
pub mod scene;

use crate::core::rtengine::RTEngine;
// use utils::geometry::Point;

use glam::Vec3A;
use ndarray::Array2;

pub fn run_lib(){

    let (p1, p2, p3, p4) = (Vec3A::new(1f32, 1f32, 1f32),
                                                       Vec3A::new(2f32, 1f32, 1f32),
                                                       Vec3A::new(3f32, 1f32, 1f32),
                                                       Vec3A::new(4f32, 1f32, 1f32));

    let mut pos_pixels = Array2::<Vec3A>::default((2,2));
    pos_pixels[[0,0]] = p1;
    pos_pixels[[0,1]] = p2;
    pos_pixels[[1,0]] = p3;
    pos_pixels[[1,1]] = p4;

    let mut rte = RTEngine{
        pos_camera : Vec3A::ZERO,
        pos_pixels : pos_pixels
        // pos_pixels : Vec::from([p1, p2, p3, p4])
    };

    rte.path_tracing();
}