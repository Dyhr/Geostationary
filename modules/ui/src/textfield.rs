use bevy::{prelude::*, utils::Instant};

pub struct TextFieldBuilder {
    text: Option<String>,
    text_style: Style,
}

pub fn build_textfield() -> TextFieldBuilder {
    TextFieldBuilder {
        text: None,
        text_style: Style::default(),
    }
}

impl TextFieldBuilder {
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((ButtonBundle { ..default() }, TextField))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    style: self.text_style,
                    text: if let Some(text) = self.text {
                        Text::from_section(
                            text,
                            TextStyle {
                                font: Handle::default(),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        )
                    } else {
                        Text::default()
                    },
                    ..default()
                });
            })
            .id()
    }
}

#[derive(Component)]
pub(crate) struct TextField;

#[derive(Component)]
pub(crate) struct Active(Instant);

#[derive(Resource)]
pub(crate) struct CurrentActive(pub Option<Entity>);

pub(crate) fn update_textfield_interactions(
    mut commands: Commands,
    query: Query<(Entity, &Children, &Interaction, &TextField), Changed<Interaction>>,
    mut window: Query<&mut Window>,
    mut current_active: ResMut<CurrentActive>,
) {
    if let Ok((entity, _, interaction, _)) = query.get_single() {
        let mut window = window.get_single_mut().unwrap();

        match *interaction {
            Interaction::Pressed => {
                if let Some(current) = current_active.0 {
                    if let Some(mut current) = commands.get_entity(current) {
                        current.remove::<Active>();
                    }
                }

                commands.entity(entity).insert(Active(Instant::now()));
                current_active.0 = Some(entity);
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Text;
            }
            Interaction::None => {
                window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

pub(crate) fn update_textfield_blinking(time: Res<Time>, mut query: Query<(&mut Text, &Active)>) {
    for (mut text, _) in query.iter_mut() {
        info!("update_textfield_blinking");
        //     timer.tick(time.delta());
        //     if timer.finished() {
        //         if text.sections()[0].style.color == Color::BLACK {
        //             text.sections_mut()[0].style.color = Color::WHITE;
        //         } else {
        //             text.sections_mut()[0].style.color = Color::BLACK;
        //         }
        //     }
    }
}
