extern crate rltk;
extern crate specs_derive;

use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};

pub use intents::*;
pub use serialization::*;

pub mod serialization;
pub mod intents;

#[derive(Component, ConvertSaveload, Clone, Debug, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

//TODO figure out way to consolidate render components
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct RenderBackground {
    pub bg: RGB,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct RenderAura {
    pub fg: RGB,
    pub glyph: u8,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Player;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Monster;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct BlocksTile;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct SuffersDamage {
    pub amount: i32,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Item;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToDrop {
    pub item: Entity,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Consumable;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct WantsToTakeTurn;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct TakesTurn {
    pub time_score: u32,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct GlobalTurn;

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct IsVisible;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct CanMove {
    pub time_cost: u32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct CanMelee {
    pub time_cost: u32,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32
}