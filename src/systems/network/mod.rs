use std::collections::HashMap;

use bevy::prelude::*;
use bevy_quinnet::{server::QuinnetServerPlugin, shared::ClientId};
use serde::{Deserialize, Serialize};

mod server;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetServerPlugin::default());
        app.add_systems(Startup, server::start_server);
        app.add_systems(
            Update,
            (
                server::handle_client_messages,
                server::handle_disconnect_events,
            ),
        );
        app.insert_resource(Users::default());
    }
}

#[derive(Resource, Debug, Clone, Default)]
struct Users {
    names: HashMap<ClientId, String>,
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
