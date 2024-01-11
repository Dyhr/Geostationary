use std::collections::HashMap;

use bevy::{app::AppExit, prelude::*};
use ui::button::build_button;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuEvent>();
        app.insert_resource(MenuEventHandlers {
            plain_handlers: HashMap::new(),
        });
        app.add_systems(Startup, (menu_setup, menu_init));
        app.add_systems(PreUpdate, menu_event_reader);
    }
}

#[derive(Event, Clone, Debug)]
pub enum MenuEvent {
    Main,
    Settings,
    Quit,
    Custom(String),
}

#[derive(Component)]
struct MenuRoot;

#[derive(Resource)]
pub struct MenuEventHandlers {
    pub plain_handlers: HashMap<String, fn() -> MenuEventHandlerResult>,
}

pub enum MenuEventHandlerResult {
    CloseMenu,
    ReplaceChildren(Vec<Entity>),
    Continue,
    Error(String),
}

fn menu_setup(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                padding: UiRect::top(Val::Px(60.0)),
                row_gap: Val::Px(30.0),
                ..default()
            },
            ..default()
        },
        MenuRoot,
    ));
}

fn menu_init(mut writer: EventWriter<MenuEvent>) {
    writer.send(MenuEvent::Main);
}

fn menu_event_reader(
    mut commands: Commands,
    query: Query<(Entity, &MenuRoot)>,
    event_handlers: Res<MenuEventHandlers>,
    mut events: EventReader<MenuEvent>,
    mut exit: EventWriter<AppExit>,
) {
    let Ok((menu_root_entity, _)) = query.get_single() else {
        panic!();
    };

    for event in events.read() {
        info!("MenuEvent read: {:?}", event);

        let result = match event {
            MenuEvent::Main => MenuEventHandlerResult::ReplaceChildren(vec![
                commands
                    .spawn(TextBundle {
                        text: Text::from_section(
                            "Geostationary",
                            TextStyle {
                                font: Handle::default(),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    })
                    .id(),
                build_button()
                    .with_text("Play")
                    .with_event(MenuEvent::Custom("PlayLocal".to_string()))
                    .build(&mut commands),
                build_button()
                    .with_text("Settings")
                    .with_event(MenuEvent::Settings)
                    .build(&mut commands),
                build_button()
                    .with_text("Quit")
                    .with_event(MenuEvent::Quit)
                    .build(&mut commands),
            ]),
            MenuEvent::Settings => MenuEventHandlerResult::ReplaceChildren(vec![
                commands
                    .spawn(TextBundle {
                        text: Text::from_section(
                            "Settings",
                            TextStyle {
                                font: Handle::default(),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    })
                    .id(),
                build_button()
                    .with_text("Back")
                    .with_event(MenuEvent::Main)
                    .build(&mut commands),
            ]),
            MenuEvent::Quit => {
                info!("Quitting");
                exit.send(AppExit);
                MenuEventHandlerResult::CloseMenu
            }
            MenuEvent::Custom(name) => {
                if let Some(handler) = event_handlers.plain_handlers.get(name) {
                    handler()
                } else {
                    MenuEventHandlerResult::Error(format!("No handler for event: {:?}", event))
                }
            }
        };

        match result {
            MenuEventHandlerResult::ReplaceChildren(children) => {
                commands
                    .entity(menu_root_entity)
                    .despawn_descendants()
                    .push_children(&children[..]);
            }
            MenuEventHandlerResult::CloseMenu => {
                commands.entity(menu_root_entity).despawn_descendants();
            }
            MenuEventHandlerResult::Continue => {}
            MenuEventHandlerResult::Error(message) => {
                error!("Error: {}", message);
            }
        }
    }
}
