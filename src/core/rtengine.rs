pub struct RTEngine{
    pub p_camera : [f32; 3],
    pub p_pixels : Vec<[f32; 3]>
}

impl RTEngine {
    pub fn run_engine(&mut self) {
        println!("{:?}", self.p_camera);
    }
}


#[test]
fn basic_instanciation() {
    let origin = [0.0f32, 0.0f32, 0.0f32];
    let origins = Vec::from([origin, origin, origin]);

    let rte = RTEngine{
        p_camera : origin,
        p_pixels : origins
    };

    assert_eq!(rte.p_camera, origin);
    assert_eq!(rte.p_pixels.len(), 3);
}