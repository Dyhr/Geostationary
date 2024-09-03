use bevy::prelude::*;

pub mod button;
pub mod lunex;
pub mod textfield;

pub use button::build_button;
pub use textfield::build_textfield;

#[derive(Default)]
pub struct UiPlugin {
    events: Vec<fn(&mut App)>,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                button::change_button_colors,
                button::button_pressed_callback,
            ),
        );
        app.add_systems(
            PreUpdate,
            (
                textfield::update_textfield_interactions,
                textfield::update_textfield_blinking,
            ),
        );

        app.insert_resource(textfield::CurrentActive(None));

        for event in &self.events {
            event(app);
        }
    }
}

impl UiPlugin {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    pub fn with_event<T: Event + Clone>(mut self) -> Self {
        self.events.push(|app| {
            app.add_event::<T>();
            app.add_systems(PreUpdate, button::process_ui_events::<T>);
        });
        self
    }
}
