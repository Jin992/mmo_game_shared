
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