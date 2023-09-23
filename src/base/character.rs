use crate::base::space::passage::*;
use crate::base::space::units::Position;
use crate::lore::locations::passages;
use crate::lore::locations::zones::*;
use crate::util::instruction::*;

pub enum Entity {
    PassageEntity(Vec<Passage>),
    CharacterEntity(Vec<Character>),
}

impl Character {
    pub fn go(&mut self, instruction: Instruction) -> Position {
        let mut axes: Vec<i32> = Vec::new();
        let mut do_move: bool = false;

        if let InstructionKind::MoveInstruct { go, x, y, z } = instruction.body {
            axes.extend_from_slice(&[x, y, z]);
            do_move = go;
        };

        if do_move {
            for (index, indexed_distance) in axes.iter().enumerate() {
                let distance = *indexed_distance;

                let limits = match index {
                    0 => (
                        self.position.area.limits.min_x,
                        self.position.area.limits.max_x,
                    ),
                    1 => (
                        self.position.area.limits.min_y,
                        self.position.area.limits.max_y,
                    ),
                    2 => (
                        self.position.area.limits.min_z,
                        self.position.area.limits.max_z,
                    ),
                    _ => unreachable!(),
                };

                let mut position = match index {
                    0 => self.position.x,
                    1 => self.position.y,
                    2 => self.position.z,
                    _ => unreachable!(),
                };

                if (position + distance) < limits.1 && (position + distance > limits.0) {
                    position += distance;
                }

                match index {
                    0 => self.position.x = position,
                    1 => self.position.y = position,
                    2 => self.position.z = position,
                    _ => unreachable!(),
                };
            }
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
            let passage: &Vec<Passage> = local_map.get(&self.position).unwrap();
            let destination_search = passage.iter().find(|&s| s.key == local_key);

            if let Some(found) = destination_search {
                self.position = found.get_destination().unwrap();
            }
        }
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

pub struct HP {
    pub current: i32,
    pub max: i32,
}

pub struct MP {
    pub current: i32,
    pub max: i32,
}
