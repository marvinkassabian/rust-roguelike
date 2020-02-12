extern crate rltk;
extern crate specs_derive;

use rltk::RGB;
use specs::prelude::*;

pub use intents::*;

pub mod intents;

#[derive(Component, Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug)]
pub struct SuffersDamage {
    pub amount: i32,
}

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug)]
pub struct WantsToDrop {
    pub item: Entity,
}

#[derive(Component, Debug)]
pub struct Consumable {}

#[derive(Component, Debug)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Component, Debug)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, Debug)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component, Debug)]
pub struct Confusion {
    pub turns: i32,
}