pub mod core;
pub mod utils;
pub mod scene;

use crate::core::rtengine::RTEngine;
use utils::geometry::Point;

pub fn run_lib(){
    let mut rte = RTEngine{
        pos_camera : Point{ ..Default::default() },
        pos_pixels : Vec::new()
    };

    rte.run_engine();
}