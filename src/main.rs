// mod color;

// fn main() {
//     println!("Hello world !");
//     let blue = color::RGBColor { r:0, g:0, b:255};
//     // let red = color::Color(255,0,0);

//     println!("{}, {}, {}", blue.r, blue.g, blue.b);
//     // println!("{}", red.0);
// }

// #[test]
// fn should_pass(){
//     assert!(1 == 1);
// }

// #[test]
// fn should_fail(){
//     unimplemented!();
// }
use rapier3d::prelude::*;

fn main() {
  let mut rigid_body_set = RigidBodySet::new();
  let mut collider_set = ColliderSet::new();

  /* Create the ground. */
  let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
  collider_set.insert(collider);

  /* Create the bounding ball. */
  let rigid_body = RigidBodyBuilder::new_dynamic()
          .translation(vector![0.0, 10.0, 0.0])
          .build();
  let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
  let ball_body_handle = rigid_body_set.insert(rigid_body);
  collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

  /* Create other structures necessary for the simulation. */
  let gravity = vector![0.0, -9.81, 0.0];
  let integration_parameters = IntegrationParameters::default();
  let mut physics_pipeline = PhysicsPipeline::new();
  let mut island_manager = IslandManager::new();
  let mut broad_phase = BroadPhase::new();
  let mut narrow_phase = NarrowPhase::new();
  let mut joint_set = ImpulseJointSet::new();
  let mut ccd_solver = CCDSolver::new();
  let physics_hooks = ();
  let event_handler = ();

  /* Run the game loop, stepping the simulation once per frame. */
  for _ in 0..200 {
    physics_pipeline.step(
      &gravity,
      &integration_parameters,
      &mut island_manager,
      &mut broad_phase,
      &mut narrow_phase,
      &mut rigid_body_set,
      &mut collider_set,
      &mut joint_set,
      &mut ccd_solver,
      &physics_hooks,
      &event_handler,
    );

    let ball_body = &rigid_body_set[ball_body_handle];
    println!(
      "Ball altitude: {}",
      ball_body.translation().y
    );
  }
}