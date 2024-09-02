use bevy::{app::AppExit, prelude::*};
use ui::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (menu_setup, menu_init));
        app.add_systems(PreUpdate, menu_event_reader);
    }
}

#[derive(Event, Clone, Debug)]
pub enum GameEvent {
    PlayLocal,
}

#[derive(Event, Clone, Debug)]
pub enum MenuEvent {
    Main,
    Settings,
    Hide,
    Quit,
}

#[derive(Component)]
struct MenuRoot;

pub enum MenuEventResult {
    CloseMenu,
    ReplaceChildren(Vec<Entity>),
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
    mut events: EventReader<MenuEvent>,
    mut exit: EventWriter<AppExit>,
) {
    let Ok((menu_root_entity, _)) = query.get_single() else {
        panic!();
    };

    for event in events.read() {
        info!("MenuEvent read: {:?}", event);

        let result = match event {
            MenuEvent::Main => MenuEventResult::ReplaceChildren(vec![
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
                build_textfield().with_text("Username").build(&mut commands),
                build_button()
                    .with_text("Play")
                    .with_event(GameEvent::PlayLocal)
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
            MenuEvent::Settings => MenuEventResult::ReplaceChildren(vec![
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
            MenuEvent::Hide => MenuEventResult::CloseMenu,
            MenuEvent::Quit => {
                info!("Quitting");
                exit.send(AppExit::Success);
                MenuEventResult::CloseMenu
            }
        };

        match result {
            MenuEventResult::ReplaceChildren(children) => {
                commands
                    .entity(menu_root_entity)
                    .despawn_descendants()
                    .push_children(&children[..]);
            }
            MenuEventResult::CloseMenu => {
                commands.entity(menu_root_entity).despawn_descendants();
            }
        }
    }
}
