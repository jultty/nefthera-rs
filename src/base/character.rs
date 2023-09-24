use crate::base::space::{passage::*, units::Position};
use crate::lore::locations::zones::*;
use crate::util::instruction::*;

impl Character {
    pub fn go(&mut self, instruction: Instruction) -> Position {
        let mut axes: Vec<i32> = Vec::new();
        let mut do_move: bool = false;

        if let InstructionKind::MoveInstruct { go, x, y, z } = instruction.body {
            axes.extend_from_slice(&[x, y, z]);
            do_move = go;
        };

        if do_move
            && self
                .position
                .area
                .limits
                .validate(self.position, axes[0], axes[1], axes[2])
        {
            self.position.x += axes[0];
            self.position.y += axes[1];
            self.position.z += axes[2];
        }
        self.position
    }

    pub fn enter_passage(&mut self, instruction: Instruction) {
        let mut local_enter: bool = false;
        let mut local_key: String = String::new();
        let mut local_map: PassageMap = PassageMap::new();

        if let InstructionKind::EnterPassageInstruct { enter, key, map } = instruction.body {
            local_enter = enter;
            local_key = key;
            local_map = map;
        }

        if local_enter {
            let passages: &Vec<Passage> = local_map.get(&self.position).unwrap();
            let destination_search = passages.iter().find(|&s| s.key == local_key);

            if let Some(found) = destination_search {
                self.position = found.get_destination().unwrap();
            }
        }
    }

    pub fn sense(&self, instruction: Instruction) -> Vec<Entity> {
        let mut sensed_entities: Vec<Entity> = Vec::new();

        // TODO should actually be mutable
        let mut local_sense: bool = false;
        // TODO should actually be the collection of several entity maps
        let mut local_world: PassageMap = PassageMap::new();
        let mut local_position = self.position;

        if let InstructionKind::SenseInstruct {
            sense,
            world,
            position,
        } = instruction.body
        {
            local_sense = sense;
            local_world = *world;

            if let Some(position) = position {
                local_position = position;
            }
        }

        if local_sense {
            let present_passages = local_world.get(&local_position).unwrap();

            // TODO sensing conditions here
            let sensed_passages = present_passages;

            sensed_entities.push(Entity::PassageEntity(sensed_passages.to_vec()));
        }
        sensed_entities
    }

    pub fn new_blank() -> Character {
        Character {
            name: "New Character".into(),
            title: "The Blank".into(),
            hp: HP {
                current: 100,
                max: 100,
            },
            mp: MP {
                current: 80,
                max: 80,
            },
            dexterity: 10,
            strength: 10,
            intelligence: 10,
            social: 10,
            perception: 10,
            grit: 10,
            position: Position {
                area: oppos_outskirts::oppos_woods::instantiate(),
                x: 0,
                y: 0,
                z: 0,
            },
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

pub enum Entity {
    PassageEntity(Vec<Passage>),
    CharacterEntity(Vec<Character>),
}

pub struct HP {
    pub current: i32,
    pub max: i32,
}

pub struct MP {
    pub current: i32,
    pub max: i32,
}
