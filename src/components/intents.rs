extern crate rltk;
extern crate specs_derive;

use rltk::Point;
use specs::prelude::*;

#[derive(Component, Debug)]
pub struct WantsToPickUp {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}
