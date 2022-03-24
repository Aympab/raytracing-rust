// mod core;
// use crate::core::rtengine::RTEngine;
// use 
pub mod core;
use crate::core::rtengine::RTEngine;

pub mod utils;
use crate::utils::geometry::Point;

pub mod scene;

fn main(){
    let mut rte = RTEngine{
        pos_camera : Point{ ..Default::default() },
        pos_pixels : Vec::new()
    };

    rte.run_engine();
}