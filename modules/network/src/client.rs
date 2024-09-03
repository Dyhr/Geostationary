use std::{thread::sleep, time::Duration};

use bevy::{app::AppExit, prelude::*};
use bevy_quinnet::{client::QuinnetClient, server::ConnectionEvent};

use super::{ClientMessage, ServerMessage, Users};

pub(crate) fn on_app_exit(app_exit_events: EventReader<AppExit>, client: Res<QuinnetClient>) {
    let Some(connection) = client.get_connection() else {
        return;
    };

    if !app_exit_events.is_empty() {
        match connection.send_message(ClientMessage::Disconnect {}) {
            Ok(()) => {}
            Err(err) => warn!("Failed do send disconnect message to server: {}", err),
        }
        // TODO Clean: event to let the async client send his last messages.
        sleep(Duration::from_secs_f32(0.1));
    }
}

pub(crate) fn handle_server_messages(mut users: ResMut<Users>, mut client: ResMut<QuinnetClient>) {
    let Some(connection) = client.get_connection_mut() else {
        return;
    };

    while let Some(message) = connection.try_receive_message::<ServerMessage>() {
        match message {
            (
                _,
                ServerMessage::ClientConnected {
                    client_id,
                    username,
                },
            ) => {
                info!("User \"{}\" joined", username);
                users.names.insert(client_id, username);
            }
            (_, ServerMessage::ClientDisconnected { client_id }) => {
                if let Some(username) = users.names.remove(&client_id) {
                    info!("{} left", username);
                } else {
                    warn!("ClientDisconnected for an unknown client_id: {}", client_id)
                }
            }
            (
                _,
                ServerMessage::InitClient {
                    client_id,
                    usernames,
                },
            ) => {
                users.self_id = client_id;
                users.names = usernames;
            }
        }
    }
}

pub(crate) fn handle_connection_events(
    mut connection_events: EventReader<ConnectionEvent>,
    client: ResMut<QuinnetClient>,
) {
    if !connection_events.is_empty() {
        let username: String = "test".to_string(); // TODO: ask for username

        info!("Joining with name: {}", username);

        match client
            .connection()
            .send_message(ClientMessage::Join { name: username })
        {
            Ok(()) => debug!("Successfully sent join message"),
            Err(err) => error!("Failed to send join message: {}", err),
        }

        connection_events.clear();
    }
}
