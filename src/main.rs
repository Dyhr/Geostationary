use bevy::{
    pbr::{DefaultOpaqueRendererMethod, DirectionalLightShadowMap},
    prelude::*,
};
use network::{NetworkEvent, NetworkPlugin};
use systems::{
    camera::CameraPlugin,
    menu::{GameEvent, MenuEvent, MenuPlugin},
    player::PlayerPlugin,
    BasePlugins,
};
use ui::UiPlugin;

mod systems;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
        .insert_resource(AmbientLight {
            color: Color::srgb(0.2, 0.2, 0.2),
            brightness: 1.0 / 3.0,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(BasePlugins {
            window_title: "Geostationary".to_string(),
        })
        .add_plugins((
            PlayerPlugin,
            CameraPlugin,
            NetworkPlugin,
            UiPlugin::new()
                .with_event::<MenuEvent>()
                .with_event::<GameEvent>(),
            MenuPlugin,
        ))
        .add_systems(Startup, (spawn_floor, spawn_light))
        .add_systems(PreUpdate, handle_game_events)
        .run();
}

fn handle_game_events(
    mut game_event: EventReader<GameEvent>,
    mut network_event: EventWriter<NetworkEvent>,
    mut menu_event: EventWriter<MenuEvent>,
) {
    for event in game_event.read() {
        info!("GameEvent read: {:?}", event);
        match event {
            GameEvent::PlayLocal => {
                network_event.send(NetworkEvent::ServerStart("127.0.0.1".to_string(), 6660));
                network_event.send(NetworkEvent::ClientConnect("127.0.0.1".to_string(), 6660));
                // TODO get result from event somehow? maybe?
                menu_event.send(MenuEvent::Hide);
            }
        }
    }
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
        mesh: meshes.add(Mesh::from(bevy_math::primitives::Plane3d::new(
            Vec3::Y,
            Vec2::new(2.0, 2.0),
        ))),
        material: materials.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3))),
        ..default()
    });
}
