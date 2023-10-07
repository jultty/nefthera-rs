use crate::base::description::Description;
use crate::base::space::{entity::*, passage::Passage, units::Position};
use crate::lore::locations::zones::*;
use crate::util::{instruction::*, print};

impl Character {
    pub fn go(&mut self, instruction: Instruction) -> Position {
        let mut axes: Vec<i32> = Vec::new();
        let mut do_move: bool = false;
        let entities = instruction.entity_map;

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

            self.sense(Instruction {
                body: InstructionKind::SenseInstruct {
                    sense: true,
                    position: Box::new(Some(self.position)),
                },
                kind: "sense".to_string(),
                entity_map: entities,
            });
        }
        self.position
    }

    pub fn enter_passage(&mut self, instruction: Instruction) {
        let mut do_enter: bool = false;
        let mut local_key: String = String::new();
        let local_entities = instruction.entity_map;

        if let InstructionKind::EnterPassageInstruct { enter, key } = instruction.body {
            do_enter = enter;
            local_key = key;
        }

        if do_enter {
            let passage_search = local_entities.get(&self.position);
            let mut passages = Vec::new();

            if let Some(found) = passage_search {
                passages = found.passages.entities.clone()
            }

            let destination_search = passages.iter().find(|&s| s.key == local_key);

            let msg: String;
            if let Some(found) = destination_search {
                self.position = found.get_destination().unwrap();
                msg = format!(
                    "Moved to {} at x {} y {} z {}\n{}",
                    self.position.area.name,
                    self.position.x,
                    self.position.y,
                    self.position.z,
                    self.position.area.description.get(),
                );
            } else {
                msg = "There is no such passage here.".to_string();
            }
            print(&msg, true);
        }
    }

    pub fn sense(&self, instruction: Instruction) -> EntityCollection {
        let mut do_sense: bool = false;
        let entities = instruction.entity_map;
        let mut sensed_entities = EntityCollection::new();
        let mut local_position = self.position;

        if let InstructionKind::SenseInstruct { sense, position } = instruction.body {
            do_sense = sense;

            if let Some(position) = *position {
                local_position = position;
            }
        }

        if do_sense {
            let mut present_passages: Vec<Passage> = Vec::new();
            let mut present_descriptions: Vec<Description> = Vec::new();
            let mut present_characters: Vec<Character> = Vec::new();

            if let Some(found) = entities.get(&local_position) {
                present_passages.extend(found.passages.entities.clone());
                present_descriptions.extend(found.descriptions.entities.clone());
                present_characters.extend(found.characters.entities.clone());
            }

            // TODO senses everything present, add conditions instead
            let mut sensed_passages = present_passages;
            let mut sensed_descriptions = present_descriptions;
            let mut sensed_characters = present_characters;

            // TODO refactor for redundancy
            if !sensed_passages.is_empty() {
                sensed_entities
                    .passages
                    .entities
                    .append(&mut sensed_passages);
            }

            if !sensed_descriptions.is_empty() {
                sensed_entities
                    .descriptions
                    .entities
                    .append(&mut sensed_descriptions);
            }

            if !sensed_characters.is_empty() {
                sensed_entities
                    .characters
                    .entities
                    .append(&mut sensed_characters);
            }

            // output message
            let mut msg: String;

            // TODO refactor for redundancy
            if sensed_entities.passages.entities.is_empty() {
                msg = "You couldn't sense passages here.".to_string();
            } else if sensed_entities.passages.entities.len() == 1 {
                msg = "You sense a passage here: ".to_string();

                msg.push_str(sensed_entities.passages.entities[0].name);
                msg.push(' ');
            } else if sensed_entities.passages.entities.len() > 1 {
                msg = "You sense passages here: ".to_string();

                for p in &sensed_entities.passages.entities {
                    msg.push_str(p.name);
                    msg.push(' ');
                }
            } else {
                panic!("Unexpected passage sensing");
            }
            print(&msg, true);

            if sensed_entities.descriptions.entities.is_empty() {
                msg = "Nothing to see here.".to_string();
            } else if sensed_entities.descriptions.entities.len() == 1 {
                msg = sensed_entities.descriptions.entities[0].get().to_string();
            } else if sensed_entities.descriptions.entities.len() > 1 {
                // TODO add paging logic
                for d in &sensed_entities.descriptions.entities {
                    msg.push_str(d.get());
                    msg.push('\n');
                }
            } else {
                panic!("Unexpected description sensing");
            }
            print(&msg, true);

            if sensed_entities.characters.entities.is_empty() {
                msg = "Nobody is here.".to_string();
            } else if sensed_entities.characters.entities.len() == 1 {
                msg = sensed_entities.characters.entities[0].name.to_string();
                msg.push_str(" is here.");
            } else if sensed_entities.characters.entities.len() > 1 {
                for c in &sensed_entities.characters.entities {
                    msg.push_str(&c.name.to_string());
                    msg.push_str(", ");
                }
                msg.push_str("are here.");
            } else {
                panic!("Unexpected character sensing");
            }
            print(&msg, true);

            if sensed_entities.descriptions.entities.is_empty() {
                msg = "Nothing to see here.".to_string();
            } else if sensed_entities.descriptions.entities.len() == 1 {
                msg = sensed_entities.descriptions.entities[0].get().to_string();
            } else if sensed_entities.descriptions.entities.len() > 1 {
                // TODO add paging logic
                for d in &sensed_entities.descriptions.entities {
                    msg.push_str(d.get());
                    msg.push('\n');
                }
            } else {
                panic!("Unexpected description sensing");
            }
            print(&msg, true);

            if sensed_entities.characters.entities.is_empty() {
                msg = "Nobody is here.".to_string();
            } else if sensed_entities.characters.entities.len() == 1 {
                msg = sensed_entities.characters.entities[0].name.to_string();
                msg.push_str(" is here.");
            } else if sensed_entities.characters.entities.len() > 1 {
                for c in &sensed_entities.characters.entities {
                    msg.push_str(&c.name.to_string());
                    msg.push_str(", ");
                }
                msg.push_str("are here.");
            } else {
                panic!("Unexpected character sensing");
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct HP {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone)]
pub struct MP {
    pub current: i32,
    pub max: i32,
}
