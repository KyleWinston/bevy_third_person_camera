use bevy::prelude::*;
use bevy_third_person_camera::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ThirdPersonCameraPlugin /* ADD THIS */))
        .add_systems(Startup, (spawn_player, spawn_world, spawn_camera))
        .run();
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneRoot(assets.load("Player.gltf#Scene0")),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player,
        ThirdPersonCameraTarget, // ADD THIS
    );

    commands.spawn(player);
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        ThirdPersonCamera::default(), // ADD THIS
    );
    commands.spawn(camera);
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.11, 0.27, 0.16))),
    );

    let light = (
        PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    );

    commands.spawn(floor);
    commands.spawn(light);
}
