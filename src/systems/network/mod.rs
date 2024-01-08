use std::collections::HashMap;

use bevy::prelude::*;
use bevy_quinnet::{client::QuinnetClientPlugin, server::QuinnetServerPlugin, shared::ClientId};
use serde::{Deserialize, Serialize};

mod client;
mod server;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            QuinnetServerPlugin::default(),
            QuinnetClientPlugin::default(),
        ));
        app.add_systems(Startup, (server::start_server, client::start_connection));
        app.add_systems(
            Update,
            (
                server::handle_client_messages,
                server::handle_disconnect_events,
                client::handle_server_messages,
                client::handle_connection_events,
            ),
        );
        app.add_systems(PostUpdate, client::on_app_exit);
        app.insert_resource(Users::default());
    }
}

#[derive(Resource, Debug, Clone, Default)]
struct Users {
    names: HashMap<ClientId, String>,
    self_id: ClientId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Join { name: String },
    Disconnect {},
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    ClientConnected {
        client_id: ClientId,
        username: String,
    },
    ClientDisconnected {
        client_id: ClientId,
    },
    InitClient {
        client_id: ClientId,
        usernames: HashMap<ClientId, String>,
    },
}
