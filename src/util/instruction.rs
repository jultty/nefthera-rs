use crate::base::space::entity::EntityMap;
use crate::base::space::units::Position;

pub struct Instruction<'a> {
    pub body: InstructionKind,
    pub kind: String,
    pub entity_map: &'a EntityMap,
}

impl Instruction<'_> {
    pub fn new_move_instruction(
        go: bool,
        x: i32,
        y: i32,
        z: i32,
        entity_map: &EntityMap,
    ) -> Instruction<'_> {
        Instruction {
            body: InstructionKind::MoveInstruct { go, x, y, z },
            kind: "go".to_string(),
            entity_map,
        }
    }

    pub fn new_quit_instruction(quit: bool, entity_map: &EntityMap) -> Instruction<'_> {
        Instruction {
            body: InstructionKind::QuitInstruct { quit },
            kind: "quit".to_string(),
            entity_map,
        }
    }

    pub fn new_sense_instruction(
        sense: bool,
        position: Option<Position>,
        entity_map: &EntityMap,
    ) -> Instruction<'_> {
        Instruction {
            body: InstructionKind::SenseInstruct {
                sense,
                position: Box::new(position),
            },
            kind: "sense".to_string(),
            entity_map,
        }
    }

    pub fn new_enter_passage_instruction(
        enter: bool,
        passage_key: String,
        entity_map: &EntityMap,
    ) -> Instruction<'_> {
        Instruction {
            body: InstructionKind::EnterPassageInstruct {
                enter,
                key: passage_key,
            },

            // TODO parse passage key from natural language
            kind: "enter_passage".to_string(),
            entity_map,
        }
    }
}

pub enum InstructionKind {
    MoveInstruct {
        go: bool,
        x: i32,
        y: i32,
        z: i32,
    },
    QuitInstruct {
        quit: bool,
    },
    SenseInstruct {
        sense: bool,
        position: Box<Option<Position>>,
    },
    EnterPassageInstruct {
        enter: bool,
        key: String,
    },
}
