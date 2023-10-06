use bevy::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Component, Clone, Default, Serialize, Deserialize)]
pub struct PlayableCharacter {
    pub position: (f32, f32, f32),
    pub name: u64,
}

#[derive(Component)]
pub struct PlayerID(pub u64);

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveEvent {
	pub forward: f32,
	pub right: f32,
}

