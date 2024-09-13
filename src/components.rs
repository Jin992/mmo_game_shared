
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_replicon::prelude::*;
/// Contains the client ID of a player.
#[derive(Component, Serialize, Deserialize)]
pub struct Player(ClientId);

impl Player {
    pub fn client_id(&self) -> ClientId {
        self.0
    }
}

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct PlayerPosition(Vec2);

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(Color);

/// A movement event for the controlled box.
#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(Vec2);