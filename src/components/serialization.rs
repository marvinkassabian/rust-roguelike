use serde::{Deserialize, Serialize};
use specs::prelude::*;

use crate::Map;

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct SerializeMe;

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct SerializationHelper {
    pub map: Map
}