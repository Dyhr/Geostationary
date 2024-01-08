use bevy::prelude::*;

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
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..default()
        },
        Player,
    ));
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query_player: Query<&mut Transform, With<Player>>,
    query_camera: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let camera = query_camera.single();

    for mut transform in query_player.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            direction += camera.forward();
        }
        if keys.pressed(KeyCode::S) {
            direction += camera.back();
        }
        if keys.pressed(KeyCode::A) {
            direction += camera.left();
        }
        if keys.pressed(KeyCode::D) {
            direction += camera.right();
        }

        direction.y = 0.0;

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * time.delta_seconds() * 10.0;
    }
}
