pub struct RTEngine{
    pub p_camera : [f32; 3],
    pub p_pixels : Vec<f32>
}

impl RTEngine {
    pub fn run_engine(&mut self) {
        println!("{:?}", self.p_camera);
    }
}