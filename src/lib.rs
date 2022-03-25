pub mod core;
pub mod utils;
pub mod scene;

use crate::core::rtengine::RTEngine;
use utils::geometry::Point;

use ndarray::Array2;

pub fn run_lib(){

    let (p1, p2, p3, p4) = (Point{x:1f32, y:1f32, z:1f32},
                                                       Point{x:2f32, y:1f32, z:1f32},
                                                       Point{x:3f32, y:1f32, z:1f32},
                                                       Point{x:4f32, y:1f32, z:1f32});

    let mut pos_pixels = Array2::<Point>::default((2,2));
    pos_pixels[[0,0]] = p1;
    pos_pixels[[0,1]] = p2;
    pos_pixels[[1,0]] = p3;
    pos_pixels[[1,1]] = p4;

    let mut rte = RTEngine{
        pos_camera : Point{ ..Default::default() },
        pos_pixels : pos_pixels
        // pos_pixels : Vec::from([p1, p2, p3, p4])
    };

    rte.path_tracing();
}