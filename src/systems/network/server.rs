use std::net::{IpAddr, Ipv4Addr};

use bevy::{
    ecs::{event::EventReader, system::ResMut},
    log::{info, warn},
};
use bevy_quinnet::{
    server::{
        certificate::CertificateRetrievalMode, ConnectionLostEvent, Endpoint, Server,
        ServerConfiguration,
    },
    shared::ClientId,
};

use super::{ClientMessage, ServerMessage, Users};

pub fn start_server(mut server: ResMut<Server>) {
    server
        .start_endpoint(
            ServerConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6660),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "127.0.0.1".into(),
            },
        )
        .unwrap();
}

pub fn handle_client_messages(mut server: ResMut<Server>, mut users: ResMut<Users>) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<ClientMessage>(client_id) {
            match message {
                ClientMessage::Join { name } => {
                    info!("{} connected", name);
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
                ClientMessage::Disconnect {} => {
                    endpoint.disconnect_client(client_id).unwrap();
                    handle_disconnect(endpoint, &mut users, client_id);
                }
            }
        }
    }
}

pub fn handle_disconnect_events(
    mut connection_lost_events: EventReader<ConnectionLostEvent>,
    mut server: ResMut<Server>,
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
