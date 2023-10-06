use bevy::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Component, Default, Serialize, Deserialize)]
pub struct PlayableCharacter {
    pub position: (f32, f32, f32),
    pub name: String,
    pub power_level: u8,
}

#[derive(Component)]
pub struct PlayerID(pub u64);

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveEvent {
	pub forward: f32,
	pub right: f32,
}

