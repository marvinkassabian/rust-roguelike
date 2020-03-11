extern crate rltk;
extern crate specs_derive;

use rltk::Point;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToPickUp {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToMove {
    pub destination: Point,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WaitCause {
    Choice,
    Confusion,
    Stun,
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToWait {
    pub cause: WaitCause,
}