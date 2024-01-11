use bevy::prelude::*;

pub struct ButtonBuilder {
    text: Option<String>,
    button_colors: ButtonColors,
    button_style: Style,
    text_style: TextStyle,
    callback: Option<ButtonCallback>,
}
pub struct ButtonBuilderWithEvent<T: Event + Clone> {
    inner: ButtonBuilder,
    event: ButtonEvent<T>,
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

    pub fn with_event<T: Event + Clone>(self, event: T) -> ButtonBuilderWithEvent<T> {
        ButtonBuilderWithEvent {
            inner: self,
            event: ButtonEvent {
                pressed: Box::new(event),
            },
        }
    }

    fn build_with_event<T: Event + Clone>(
        self,
        commands: &mut Commands,
        event: ButtonEvent<T>,
    ) -> Entity {
        let mut entity = commands.spawn((
            ButtonBundle {
                style: self.button_style,
                ..default()
            },
            self.button_colors,
        ));

        if let Some(callback) = self.callback {
            entity.insert(callback);
        }

        entity.insert(event);

        entity.with_children(|parent| match &self.text {
            Some(text) => {
                parent.spawn(TextBundle {
                    text: Text::from_section(text, self.text_style),
                    ..default()
                });
            }
            None => {}
        });

        entity.id()
    }
    pub fn build(self, commands: &mut Commands) -> Entity {
        let mut entity = commands.spawn((
            ButtonBundle {
                style: self.button_style,
                ..default()
            },
            self.button_colors,
        ));

        if let Some(callback) = self.callback {
            entity.insert(callback);
        }

        entity.with_children(|parent| match &self.text {
            Some(text) => {
                parent.spawn(TextBundle {
                    text: Text::from_section(text, self.text_style),
                    ..default()
                });
            }
            None => {}
        });

        entity.id()
    }
}

impl<T: Event + Clone> ButtonBuilderWithEvent<T> {
    pub fn build(self, commands: &mut Commands) -> Entity {
        self.inner.build_with_event(commands, self.event)
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
pub(crate) struct ButtonCallback {
    pub pressed: Box<fn()>,
}

pub(crate) fn button_pressed_callback(
    mut query: Query<(&Interaction, &ButtonCallback), Changed<Interaction>>,
) {
    if let Ok((interaction, callback)) = query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            (callback.pressed)();
        }
    }
}

#[derive(Component, Clone)]
pub(crate) struct ButtonEvent<T: Event + Clone> {
    pub(crate) pressed: Box<T>,
}

pub(crate) fn process_ui_events<T: Event + Clone>(
    mut query: Query<(&Interaction, &ButtonEvent<T>), Changed<Interaction>>,
    mut writer: EventWriter<T>,
) {
    if let Ok((interaction, callback)) = query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            writer.send(callback.pressed.as_ref().clone());
        }
    }
}
