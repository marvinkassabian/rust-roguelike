use std::fs;
use std::fs::File;
use std::path::Path;

use specs::{Builder, Entity, World, WorldExt};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker, SimpleMarkerAllocator};

use crate::{Map, SerializeMe};
use crate::components::*;

const SAVE_FILE_PATH: &str = "./save_game.json";

macro_rules! serialize_individually {
    ($ecs:expr, $serializer:expr, $entities:expr, $simple_markers:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$entities,
            &$simple_markers,
            &mut $serializer,
        )
        .unwrap();
        )*
    };
}

pub fn save_game(ecs: &mut World) {
    let map_copy = ecs.get_mut::<Map>().unwrap().clone();
    let save_helper = ecs
        .create_entity()
        .with(SerializationHelper { map: map_copy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    {
        let (entities, simple_markers) = (ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>());

        let writer = File::create(SAVE_FILE_PATH).unwrap();
        let mut serializer = serde_json::Serializer::new(writer);

        serialize_individually!(
            ecs, serializer, entities, simple_markers,
            Position,
            Renderable,
            Player,
            Viewshed,
            Monster,
            Name,
            BlocksTile,
            CombatStats,
            WantsToMelee,
            SuffersDamage,
            Item,
            InBackpack,
            WantsToPickUp,
            WantsToUseItem,
            WantsToDrop,
            Consumable,
            ProvidesHealing,
            Ranged,
            InflictsDamage,
            AreaOfEffect,
            Confusion,
            WantsToTakeTurn,
            TakesTurn,
            GlobalTurn,
            WantsToMove,
            WantsToWait,
            IsVisible,
            CanMove,
            CanMelee,
            ParticleLifetime,
            RenderBackground,
            RenderAura,
            SerializationHelper
        );
    }

    ecs.delete_entity(save_helper).expect("Crash on cleanup");
}

pub fn does_save_exist() -> bool {
    Path::new(SAVE_FILE_PATH).exists()
}

macro_rules! deserialize_individually {
    ($ecs:expr, $deserializer:expr, $entities:expr, $marker:expr, $allocator:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $entities,
            &mut $marker,
            &mut $allocator,
            &mut $deserializer,
        )
        .unwrap();
        )*
    };
}

pub fn load_game(ecs: &mut World) {
    {
        let mut to_delete = Vec::new();
        for entity in ecs.entities().join() {
            to_delete.push(entity);
        }
        for entity in to_delete.iter() {
            ecs.delete_entity(*entity).expect("Deletion failed");
        }
    }

    let save_file_data = fs::read_to_string(SAVE_FILE_PATH).unwrap();
    let mut deserializer = serde_json::Deserializer::from_str(&save_file_data);

    {
        let mut entities = &mut ecs.entities();
        let mut serializer = &mut ecs.write_storage::<SimpleMarker<SerializeMe>>();
        let mut allocator = &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>();

        deserialize_individually!(
            ecs, deserializer, entities, serializer, allocator,
            Position,
            Renderable,
            Player,
            Viewshed,
            Monster,
            Name,
            BlocksTile,
            CombatStats,
            WantsToMelee,
            SuffersDamage,
            Item,
            InBackpack,
            WantsToPickUp,
            WantsToUseItem,
            WantsToDrop,
            Consumable,
            ProvidesHealing,
            Ranged,
            InflictsDamage,
            AreaOfEffect,
            Confusion,
            WantsToTakeTurn,
            TakesTurn,
            GlobalTurn,
            WantsToMove,
            WantsToWait,
            IsVisible,
            CanMove,
            CanMelee,
            ParticleLifetime,
            RenderBackground,
            RenderAura,
            SerializationHelper
        );
    }

    let mut delete_me: Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helpers = ecs.read_storage::<SerializationHelper>();
        let players = ecs.read_storage::<Player>();
        let positions = ecs.read_storage::<Position>();

        for (entity, helper) in (&entities, &helpers).join() {
            let mut world_map = ecs.write_resource::<Map>();
            *world_map = helper.map.clone();
            world_map.tile_content = vec![Vec::new(); world_map.count()];
            delete_me = Some(entity);
        }

        for (entity, _player, position) in (&entities, &players, &positions).join() {
            let mut player_position = ecs.write_resource::<rltk::Point>();
            *player_position = rltk::Point::new(position.x, position.y);
            let mut player_entity = ecs.write_resource::<Entity>();
            *player_entity = entity;
        }
    }

    ecs.delete_entity(delete_me.unwrap()).expect("Unable to delete helper");
}

pub fn delete_save() {
    if Path::new(SAVE_FILE_PATH).exists() {
        std::fs::remove_file(SAVE_FILE_PATH).expect("Unable to delete file");
    }
}