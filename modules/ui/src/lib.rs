use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, change_button_colors);
    }
}

#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn change_button_colors(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonColors), Changed<Interaction>>,
) {
    if let Ok((interaction, mut background_color, button_colors)) = query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = button_colors.pressed;
            }
            Interaction::Hovered => {
                background_color.0 = button_colors.hovered;
            }
            Interaction::None => {
                background_color.0 = button_colors.normal;
            }
        }
    }
}
