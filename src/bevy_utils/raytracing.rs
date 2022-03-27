use bevy::prelude::*;

use super::components::BevyLightC;

///Gets all the positions and objects components from the scene
/// and feed them to the raytracing engine
pub fn compute_rt(
    keyboard_input: Res<Input<KeyCode>>,
    query : Query<(& Transform, With<BevyLightC>)>
){
    // let Ok((tr, _)) = query.single_mut();
    
    let (tr, _) = query.single();

    if keyboard_input.just_released(KeyCode::Space){
        println!("Light position : {}, {}, {}",
            tr.translation.x,
            tr.translation.y,
            tr.translation.z);
    }

}