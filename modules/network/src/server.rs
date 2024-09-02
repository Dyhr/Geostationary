use bevy::{
    ecs::{event::EventReader, system::ResMut},
    log::{info, warn},
};
use bevy_quinnet::{
    server::{ConnectionLostEvent, Endpoint, QuinnetServer},
    shared::ClientId,
};

use super::{ClientMessage, ServerMessage, Users};

pub fn handle_client_messages(mut server: ResMut<QuinnetServer>, mut users: ResMut<Users>) {
    let Some(endpoint) = server.get_endpoint_mut() else {
        return;
    };
    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<ClientMessage>(client_id) {
            match message {
                (_, ClientMessage::Join { name }) => {
                    info!("User \"{}\" connected", name);
                    users.names.insert(client_id, name);
                    endpoint
                        .send_message(
                            client_id,
                            ServerMessage::InitClient {
                                client_id,
                                usernames: users.names.clone(),
                            },
                        )
                        .unwrap();
                }
                (_, ClientMessage::Disconnect {}) => {
                    endpoint.disconnect_client(client_id).unwrap();
                    handle_disconnect(endpoint, &mut users, client_id);
                }
            }
        }
    }
}

pub fn handle_disconnect_events(
    mut connection_lost_events: EventReader<ConnectionLostEvent>,
    mut server: ResMut<QuinnetServer>,
    mut users: ResMut<Users>,
) {
    for client in connection_lost_events.read() {
        handle_disconnect(server.endpoint_mut(), &mut users, client.id);
    }
}

fn handle_disconnect(endpoint: &mut Endpoint, users: &mut ResMut<Users>, client_id: ClientId) {
    if let Some(username) = users.names.remove(&client_id) {
        endpoint
            .send_group_message(
                users.names.keys(),
                ServerMessage::ClientDisconnected { client_id },
            )
            .unwrap();
        info!("{} disconnected", username);
    } else {
        warn!(
            "Received a Disconnect from an unknown or disconnected client: {}",
            client_id
        )
    }
}
