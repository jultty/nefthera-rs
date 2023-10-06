use super::units::Position;
use crate::base::character::Character;
use crate::base::description::Description;
use crate::base::space::passage::Passage;
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

impl Default for EntityCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct EntityGroup<T> {
    pub entities: Vec<T>,
}
