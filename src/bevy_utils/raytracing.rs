use bevy::prelude::*;
// use bevy::asset::
use super::components::{BevyLightC, BevyCameraC, BevySphereC};

///Gets all the positions and objects components from the scene
/// and feed them to the raytracing engine
pub fn compute_rt(
    keyboard_input: Res<Input<KeyCode>>,
    light_query : Query<(&Transform, With<BevyLightC>)>,
    cam_query : Query<(&Transform, With<BevyCameraC>)>,
    object_query : Query<(&Transform, With<BevySphereC>)>
    // object_query : Query<(&Transform, &Handle<Mesh>, &Handle<StandardMaterial>, With<BevySphereC>)>
){
    let (light_tr, _) = light_query.single();
    let (cam_tr, _) = cam_query.single();

    if keyboard_input.just_released(KeyCode::Space){
        println!("Light position : {}, {}, {}",
            light_tr.translation.x,
            light_tr.translation.y,
            light_tr.translation.z);

        println!("Cam position : {}, {}, {}",
            cam_tr.translation.x,
            cam_tr.translation.y,
            cam_tr.translation.z);
        
            println!("Spheres :");

        let mut i = 0;
        // for (tr, mesh, mat, _) in object_query.iter(){
        for (tr, _) in object_query.iter(){
            i += 1;
            println!("{}:", i);
    
            // let color = a.get_with_id(mat.id);
            println!("  > Pos : {}, {}, {}", tr.translation.x, tr.translation.y, tr.translation.z);
            // println!("  > Color : {}, {}, {}", mat.albedo.r(), mat.albedo.g(), mat.albedo.b())
        }
    }

    //TODO : Call Raytracing engine with values gotten from bevy

}