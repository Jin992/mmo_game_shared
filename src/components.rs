
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_replicon::prelude::*;
/// Contains the client ID of a player.
#[derive(Component, Serialize, Deserialize)]
pub struct Player {
   pub client_id: ClientId
}

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct PlayerPosition {
    pub position: Vec2,
}

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor {
    pub color: Color,
}

/// A movement event for the controlled box.
#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection {
    pub direction: Vec2,
}


pub const PROTOCOL_ID: u64 = 0;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    position: PlayerPosition,
    color: PlayerColor,
    replicated: Replicated,
}

impl PlayerBundle {
    pub fn new(client_id: ClientId, position: Vec2, color: Color) -> Self {
        Self {
            player: Player{client_id},
            position: PlayerPosition{position},
            color: PlayerColor{color},
            replicated: Replicated,
        }
    }
}