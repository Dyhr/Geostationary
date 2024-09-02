use bevy::prelude::*;
use bevy_math::primitives;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(primitives::Capsule3d::default())),
            material: materials.add(StandardMaterial::from(Color::srgb(0.5, 0.5, 1.0))),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..default()
        },
        Player,
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query_player: Query<&mut Transform, With<Player>>,
    query_camera: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let camera = query_camera.single();

    for mut transform in query_player.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += camera.forward().normalize();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += camera.back().normalize();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += camera.left().normalize();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += camera.right().normalize();
        }

        direction.y = 0.0;

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * time.delta_seconds() * 10.0;
    }
}
