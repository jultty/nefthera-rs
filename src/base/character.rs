use crate::base::space::{passage::*, units::Position};
use crate::lore::locations::zones::*;
use crate::util::{instruction::*, print};

// TODO extract this

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

            let msg: String = format!(
                "Moved to x {} y {} z {}",
                self.position.x, self.position.y, self.position.z
            );
            print(&msg, true);
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
            let passage_search = local_map.get(&self.position);
            let mut passages = Vec::new();

            if let Some(found) = passage_search {
                passages = found.to_vec();
            }

            let destination_search = passages.iter().find(|&s| s.key == local_key);

            let msg: String;
            if let Some(found) = destination_search {
                self.position = found.get_destination().unwrap();
                msg = format!(
                    "Moved to {} at x {} y {} z {}",
                    self.position.area.name, self.position.x, self.position.y, self.position.z
                );
            } else {
                msg = "There is no such passage here.".to_string();
            }
            print(&msg, true);
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

            // TODO senses everything present, add conditions instead
            let sensed_passages = present_passages;

            sensed_entities.push(Entity::PassageEntity(sensed_passages.to_vec()));

            // output message
            let mut msg: String;

            if sensed_entities.is_empty() {
                msg = "You couldn't sense anything here.".to_string();
            } else {
                if !sensed_passages.is_empty() {
                    msg = "You sense the following passages here: ".to_string();

                    for p in sensed_passages {
                        msg.push_str(p.name);
                        msg.push(' ');
                    }
                } else {
                    msg = "You sense something unknown in this place.".to_string();
                }
            }
            print(&msg, true);
        }
        sensed_entities
    }

    pub fn new_blank() -> Character {
        Character {
            name: "Unnamed".into(),
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
                x: -10500,
                y: -71500,
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
