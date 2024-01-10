use bevy::prelude::*;

pub mod button;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, button::change_button_colors);
        app.add_systems(PreUpdate, button::button_pressed_events);
    }
}
