use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use super::{components::{BevyLightC, BevyCameraC, BevySphereC}, spawns::MyRaycastSet};
// use super::spawns::MyRaycastSet;
use bevy_mod_raycast::{Intersection, Ray3d, RayCastSource, ray_intersection_over_mesh};

///Gets all the positions and objects components from the scene
/// and feed them to the raytracing engine
pub fn compute_rt(
    keyboard_input: Res<Input<KeyCode>>,
    light_query : Query<(&Transform, With<BevyLightC>)>,
    cam_query : Query<(&Transform, With<BevyCameraC>)>,
    object_query : Query<(&Transform, With<BevySphereC>)>,
    raysource : Query<&RayCastSource<MyRaycastSet>>,
    // intersection_query : Query<&Intersection>
    // object_query : Query<(&Transform, &Handle<Mesh>, &Handle<StandardMaterial>, With<BevySphereC>)>
){
    let (light_tr, _) = light_query.single();
    let (cam_tr, _) = cam_query.single();

    if keyboard_input.just_released(KeyCode::Space){
    //     println!("Light position : {}, {}, {}",
    //         light_tr.translation.x,
    //         light_tr.translation.y,
    //         light_tr.translation.z);

    //     println!("Cam position : {}, {}, {}",
    //         cam_tr.translation.x,
    //         cam_tr.translation.y,
    //         cam_tr.translation.z);
        
    //         println!("Spheres :");

    //     let mut i = 0;
    //     // for (tr, mesh, mat, _) in object_query.iter(){
    //     for (tr, _) in object_query.iter(){
    //         i += 1;
    //         println!("{}:", i);
    
    //         // let color = a.get_with_id(mat.id);
    //         println!("  > Pos : {}, {}, {}", tr.translation.x, tr.translation.y, tr.translation.z);
    //         // println!("  > Color : {}, {}, {}", mat.albedo.r(), mat.albedo.g(), mat.albedo.b())
    //     }

    let raycast_source = raysource.single();
    
    if let Some(top_intersection) = raycast_source.intersect_top() {
        let from = cam_tr.translation;
        let to = top_intersection.1.position();
        let ray_direction = (to - from).normalize();

        let ray = Ray3d::new(from, ray_direction);
    }        
        // let ray = Ray3d::new(cam_tr.translation, //from
        //     cam_tr.rotation.normalize());  //ray_direction);

    // Report intersections
    // println!("Intersections is empty : {}", intersection_query.is_empty());
    // for intersection in intersection_query.iter() {
    //     info!(
    //         "Distance {:?}, Position {:?}",
    //         intersection.distance(),
    //         intersection.position()
    //         );
    //     }
    }
    //TODO : Call Raytracing engine with values gotten from bevy
}

// Marker struct for the path origin, shown by a cyan sphere
#[derive(Component)]
pub struct PathOrigin;

// Marker struct for obstacles
#[derive(Component)]
pub struct PathObstacle;
// Marker struct for the intersection point
#[derive(Component)]
pub struct PathObstaclePoint;

#[allow(clippy::type_complexity)]
// Check the path between origin and mouse cursor position
pub fn check_path(
    keyboard_input: Res<Input<KeyCode>>,
    mut from: Query<
        &mut Transform,
        (
            With<PathOrigin>,
            Without<PathObstacle>,
            Without<PathObstaclePoint>,
        ),
    >,
    to: Query<&RayCastSource<MyRaycastSet>>,
    obstacles: Query<(&Handle<Mesh>, &Transform), With<PathObstacle>>,
    meshes: Res<Assets<Mesh>>,
    mut intersection_point: Query<
        (&mut Transform, &mut Visibility),
        (
            With<PathObstaclePoint>,
            Without<PathObstacle>,
            Without<PathOrigin>,
        ),
    >,
) {
    if keyboard_input.just_released(KeyCode::E){
        if let Ok(mut origin_transform) = from.get_single_mut() {
            let raycast_source = to.single();
            // let mut pointer = pointer.single_mut();
            if let Some(top_intersection) = raycast_source.intersect_top() {
                let from = origin_transform.translation;
                let to = top_intersection.1.position();
                let ray_direction = (to - from).normalize();

                // Rotate the direction indicator
                if Vec3::Z.angle_between(ray_direction) > FRAC_PI_2 {
                    origin_transform.rotation =
                        Quat::from_rotation_y(Vec3::X.angle_between(ray_direction));
                } else {
                    origin_transform.rotation =
                        Quat::from_rotation_y(-Vec3::X.angle_between(ray_direction));
                }

                let ray = Ray3d::new(from, ray_direction);
                // if let Ok(mut text) = status_query.get_single_mut() {
                    if let Ok((mut intersection_transform, mut visible)) =
                        intersection_point.get_single_mut()
                    {
                        // Set everything as OK in case there are no obstacle in path
                        // text.sections[1].value = "Direct!".to_string();
                        // text.sections[1].style.color = Color::GREEN;
                        println!("Direct !");
                        visible.is_visible = false;

                        let mut closest_hit = f32::MAX;

                        // Check for an obstacle on path
                        for (mesh_handle, transform) in obstacles.iter() {
                            if let Some(mesh) = meshes.get(mesh_handle) {
                                let mesh_to_world = transform.compute_matrix();

                                // Check for intersection with this obstacle
                                if let Some(intersection) =
                                    ray_intersection_over_mesh(mesh, &mesh_to_world, &ray)
                                {
                                    // There was an intersection, check if it is before the cursor
                                    // on the ray
                                    let hit_distance = intersection.distance();
                                    let cursor_distance = from.distance(to);
                                    if hit_distance < cursor_distance && hit_distance < closest_hit {
                                        // text.sections[1].value = "Obstructed!".to_string();
                                        // text.sections[1].style.color = Color::RED;
                                        println!("Obstructed !");
                                        intersection_transform.translation = intersection.position();
                                        visible.is_visible = true;
                                        closest_hit = hit_distance;
                                    }
                                }
                            }
                        }

                        // pointer.scale = Vec3::new(closest_hit / 2.0, 0.05, 0.05);
                        // pointer.translation = Vec3::new(closest_hit / 2.0, 0.0, 0.0);
                    }
                // }
            }
        }
    }
}
