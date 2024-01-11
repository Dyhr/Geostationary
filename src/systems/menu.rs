use bevy::{app::AppExit, prelude::*};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuEvent>();
        app.add_systems(Startup, menu_setup);
        app.add_systems(PreUpdate, menu_event_reader);
    }
}

#[derive(Event, Clone)]
pub enum MenuEvent {
    Main,
    Settings,
    Quit,
    Custom(String),
}

#[derive(Component)]
struct MenuRoot;

// TODO add a startup system that creates a root node for the ui and adds a new MenuRoot component to it so we can track it
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

fn menu_event_reader(
    mut commands: Commands,
    query: Query<(Entity, &MenuRoot)>,
    mut events: EventReader<MenuEvent>,
    mut exit: EventWriter<AppExit>,
) {
    let Ok((menu_root_entity, _)) = query.get_single() else {
        panic!();
    };
    let mut commands = commands.entity(menu_root_entity);

    for event in events.read() {
        match event {
            MenuEvent::Main => {
                // TODO: Switch to the main menu state
            }
            MenuEvent::Settings => {
                // TODO: Switch to the settings menu state
            }
            MenuEvent::Quit => {
                info!("Quitting");
                exit.send(AppExit);
            }
            MenuEvent::Custom(name) => {
                // TODO: Handle custom menu event
            }
        }
    }
}
