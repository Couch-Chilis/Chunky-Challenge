use std::{collections::BTreeSet, fs};

use anyhow::Context;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::ensure_chunky_dir;

#[derive(Default, Deserialize, Resource, Serialize)]
pub struct GameState {
    #[serde(skip)]
    pub current_level: u16,

    #[serde(skip)]
    pub previous_level: Option<u16>,

    finished_levels: BTreeSet<u16>,
}

impl GameState {
    pub fn mark_level_finished(&mut self, level: u16) {
        self.finished_levels.insert(level);

        self.save();
    }

    pub fn is_finished_level(&self, level: u16) -> bool {
        self.finished_levels.contains(&level)
    }

    /// Loads game state from disk, or returns `Self::default()` if no
    /// game state could be loaded.
    pub fn load() -> Self {
        fs::read(ensure_chunky_dir().join("game_state.json"))
            .context("Can't read file")
            .and_then(|json| Self::from_json(&json))
            .map_err(|err| println!("Can't load game state: {err}"))
            .unwrap_or_default()
    }

    pub fn is_in_hub(&self) -> bool {
        self.current_level == 0
    }

    pub fn set_current_level(&mut self, level: u16) {
        if level > 0 || self.current_level > 0 {
            self.previous_level = Some(self.current_level);
            self.current_level = level;
        }
    }

    /// Saves game state to disk.
    ///
    /// This is called automatically on drop.
    fn save(&self) {
        self.to_json()
            .and_then(|json| {
                fs::write(ensure_chunky_dir().join("game_state.json"), json)
                    .map_err(anyhow::Error::from)
            })
            .unwrap_or_else(|err| println!("Can't save game state: {err}"));
    }

    /// Serializes the game state to JSON.
    fn to_json(&self) -> Result<Vec<u8>, anyhow::Error> {
        serde_json::to_vec(self).map_err(anyhow::Error::from)
    }

    /// Parses game state from JSON.
    fn from_json(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        serde_json::from_slice(bytes).map_err(anyhow::Error::from)
    }
}
