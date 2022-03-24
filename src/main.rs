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
        p_camera : Point{ ..Default::default() },
        p_pixels : Vec::new()
    };

    rte.run_engine();
}