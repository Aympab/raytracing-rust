// mod core;
// use crate::core::rtengine::RTEngine;
// use 
mod core;
use crate::core::rtengine::RTEngine;

fn main(){
    let mut rte = RTEngine{
        p_camera : [10.0f32, 0.0f32, 0.0f32],
        p_pixels : Vec::new()
    };

    rte.run_engine();
}