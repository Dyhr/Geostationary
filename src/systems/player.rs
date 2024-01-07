use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
        material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        ..default()
    });
}