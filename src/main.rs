use bevy::prelude::*;
use systems::{player::PlayerPlugin, camera::CameraPlugin};

mod systems;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin))
        .add_systems(Startup, (spawn_floor, spawn_light))
        .run();
}


fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 8.0, 0.0)),
        ..default()
    });
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(20.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}
