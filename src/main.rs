mod core;

fn main(){
    let mut rte = core::rtengine::RTEngine{
        p_camera : [0.0f32, 0.0f32, 0.0f32],
        p_pixels : Vec::new()
    };

    rte.run_engine();
}