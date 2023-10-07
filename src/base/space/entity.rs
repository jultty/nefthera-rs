use super::units::Position;
use crate::base::character::Character;
use crate::base::description::Description;
use crate::base::space::passage::Passage;
use crate::lore::characters;
use crate::lore::descriptions;
use crate::lore::locations::passages;
use std::collections::HashMap;

pub type EntityMap = HashMap<Position, EntityCollection>;

#[derive(Clone)]
pub struct EntityCollection {
    pub passages: EntityGroup<Passage>,
    pub descriptions: EntityGroup<Description>,
    pub characters: EntityGroup<Character>,
}

impl EntityCollection {
    pub fn new() -> EntityCollection {
        EntityCollection {
            passages: EntityGroup::<Passage> {
                entities: Vec::<Passage>::new(),
            },
            characters: EntityGroup::<Character> {
                entities: Vec::<Character>::new(),
            },
            descriptions: EntityGroup::<Description> {
                entities: Vec::<Description>::new(),
            },
        }
    }
}

pub fn new_populated() -> EntityMap {
    let mut map = EntityMap::new();
    let all_passages = passages::get_vector();
    let all_characters = characters::get_vector();
    let all_descriptions = descriptions::get_vector();

    all_passages.into_iter().for_each(|p| {
        let origin = p.get_origin().unwrap();

        match map.get_mut(&origin) {
            Some(collection) => collection.passages.entities.push(p),
            None => {
                let mut collection = EntityCollection::new();
                collection.passages.entities.push(p);
                map.insert(origin, collection);
            }
        };
    });

    all_characters.into_iter().for_each(|c| {
        let position = c.position;

        match map.get_mut(&position) {
            Some(collection) => collection.characters.entities.push(c),
            None => {
                let mut collection = EntityCollection::new();
                collection.characters.entities.push(c);
                map.insert(position, collection);
            }
        };
    });

    all_descriptions.into_iter().for_each(|d| {
        let position = d.0;

        match map.get_mut(&position) {
            Some(collection) => collection.descriptions.entities.push(d.1),
            None => {
                let mut collection = EntityCollection::new();
                collection.descriptions.entities.push(d.1);
                map.insert(position, collection);
            }
        };
    });

    map
}

impl Default for EntityCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct EntityGroup<T> {
    pub entities: Vec<T>,
}
