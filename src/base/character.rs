use crate::base::space::passage::Passage;
use crate::base::space::units::Position;
use crate::lore::locations::passages;

impl Character {
    pub fn go(&mut self, x: i32, y: i32, z: i32) -> Position {
        if self.position.area.limits.validate(self.position, x, y, z) {
            self.position.x += x;
            self.position.y += y;
            self.position.z += z;
        };
        self.position
    }

    pub fn enter_passage(&mut self, passage: Passage) {
        if passage.get_destination().is_some() {
            self.position = passage.get_destination().unwrap();
        }

        enum Entity {
            PassageEntity(Vec<Passage>),
            CharacterEntity(Vec<Character>),
        }

        pub fn sense(position: Position) -> Vec<Entity> {
            let mut sensed_entities: Vec<Entity> = Vec::new();

            // TODO should actually be mutable
            let passage_map = passages::populate();

            let present_passages = passage_map.get(&position).unwrap();

            // TODO sensing conditions here
            let sensed_passages = present_passages;

            sensed_entities.push(Entity::PassageEntity(sensed_passages.to_vec()));

            sensed_entities
        }
    }
}

pub struct Character {
    pub name: Box<str>,
    pub title: Box<str>,
    pub hp: HP,
    pub mp: MP,
    pub dexterity: i32,
    pub strength: i32,
    pub intelligence: i32,
    pub social: i32,
    pub perception: i32,
    pub grit: i32,
    pub position: Position,
}

pub struct HP {
    pub current: i32,
    pub max: i32,
}

pub struct MP {
    pub current: i32,
    pub max: i32,
}
