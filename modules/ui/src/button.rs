use bevy::prelude::*;

pub struct ButtonBuilder {
    text: Option<String>,
    button_colors: ButtonColors,
    button_style: Style,
    text_style: TextStyle,
        callback: Option<ButtonCallback>,
    }

    pub fn build_button() -> ButtonBuilder {
        ButtonBuilder {
            text: None,
            button_colors: ButtonColors {
                normal: Color::rgb(0.4, 0.4, 0.4),
                hovered: Color::rgb(0.6, 0.6, 0.6),
                pressed: Color::rgb(0.2, 0.2, 0.2),
            },
            button_style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            text_style: TextStyle {
                font: Handle::default(),
                font_size: 50.0,
                color: Color::BLACK,
            },
            callback: None,
        }
    }

    impl ButtonBuilder {
        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_string());
            self
        }

        pub fn with_colors(mut self, normal: Color, hovered: Color, pressed: Color) -> Self {
            self.button_colors = ButtonColors {
                normal,
                hovered,
                pressed,
            };
            self
        }

        pub fn with_callback(mut self, callback: Box<fn()>) -> Self {
            self.callback = Some(ButtonCallback { pressed: callback });
            self
        }

        pub fn build(self, parent: &mut ChildBuilder<'_, '_, '_>) {
            if let Some(callback) = self.callback {
                parent.spawn((
                    ButtonBundle {
                        style: self.button_style,
                        ..default()
                    },
                    callback,
                    self.button_colors,
                ))
            } else {
                parent.spawn((
                    ButtonBundle {
                        style: self.button_style,
                        ..default()
                    },
                    self.button_colors,
                ))
            }
            .with_children(|parent| match &self.text {
                Some(text) => {
                    parent.spawn(TextBundle {
                        text: Text::from_section(text, self.text_style),
                        ..default()
                    });
                }
                None => {}
            });
        }
    }

    #[derive(Component)]
    pub struct ButtonColors {
        pub normal: Color,
        pub hovered: Color,
        pub pressed: Color,
    }

    pub(crate) fn change_button_colors(
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

    #[derive(Component)]
    pub struct ButtonCallback {
        pub pressed: Box<fn()>,
    }

    pub(crate) fn button_pressed_events(
        mut query: Query<(&Interaction, &ButtonCallback), Changed<Interaction>>,
    ) {
        if let Ok((interaction, callback)) = query.get_single_mut() {
            if *interaction == Interaction::Pressed {
                (callback.pressed)();
            }
        }
    }
