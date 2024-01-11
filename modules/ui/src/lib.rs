use bevy::prelude::*;

pub mod button;

#[derive(Default)]
pub struct UiPlugin {
    events: Vec<fn(&mut App)>,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, button::change_button_colors);
        app.add_systems(PreUpdate, button::button_pressed_callback);

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
