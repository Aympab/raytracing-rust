//Example of using the asset_server.load with a gltf file
use bevy::prelude::*;
use bevy_mod_picking::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_system(animate_light_direction)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_scene(asset_server.load("FlightHelmet\\FlightHelmet.gltf#Scene0"));
    // commands.spawn_scene(asset_server.load("skull\\scene.gltf#Scene0"));
    commands.spawn_scene(asset_server.load("car-kit-3d-model\\Models\\GLTF Format\\ambulance.glb#Scene0"));
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(11.7,
                                       10.7,
                                       11.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()
    }).insert_bundle(PickingCameraBundle::default());
    const HALF_SIZE: f32 = 100.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}