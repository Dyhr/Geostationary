use std::{thread::sleep, time::Duration};

use bevy::{app::AppExit, prelude::*};
use bevy_quinnet::{
    client::{
        certificate::CertificateVerificationMode, connection::ConnectionConfiguration, Client,
    },
    server::ConnectionEvent,
};

use super::{ClientMessage, ServerMessage, Users};

pub fn on_app_exit(app_exit_events: EventReader<AppExit>, client: Res<Client>) {
    if !app_exit_events.is_empty() {
        client
            .connection()
            .send_message(ClientMessage::Disconnect {})
            .unwrap();
        // TODO Clean: event to let the async client send his last messages.
        sleep(Duration::from_secs_f32(0.1));
    }
}

pub fn handle_server_messages(mut users: ResMut<Users>, mut client: ResMut<Client>) {
    while let Some(message) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::ClientConnected {
                client_id,
                username,
            } => {
                info!("{} joined", username);
                users.names.insert(client_id, username);
            }
            ServerMessage::ClientDisconnected { client_id } => {
                if let Some(username) = users.names.remove(&client_id) {
                    info!("{} left", username);
                } else {
                    warn!("ClientDisconnected for an unknown client_id: {}", client_id)
                }
            }
            ServerMessage::InitClient {
                client_id,
                usernames,
            } => {
                users.self_id = client_id;
                users.names = usernames;
            }
        }
    }
}

pub fn start_connection(mut client: ResMut<Client>) {
    client
        .open_connection(
            ConnectionConfiguration::from_strings("127.0.0.1:6660", "0.0.0.0:0").unwrap(),
            CertificateVerificationMode::SkipVerification,
        )
        .unwrap();
}

pub fn handle_connection_events(
    mut connection_events: EventReader<ConnectionEvent>,
    client: ResMut<Client>,
) {
    if !connection_events.is_empty() {
        let username: String = "test".to_string(); // TODO: ask for username

        info!("Joining with name: {}", username);

        client
            .connection()
            .send_message(ClientMessage::Join { name: username })
            .unwrap();

        connection_events.clear();
    }
}
