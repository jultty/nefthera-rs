use crate::base::space::passage::*;
use crate::base::space::units::Position;

pub struct Instruction {
    pub body: InstructionKind,
    pub kind: String,
}

impl Instruction {
    pub fn new_move_instruction(go: bool, x: i32, y: i32, z: i32) -> Instruction {
        Instruction {
            body: InstructionKind::MoveInstruct { go, x, y, z },
            kind: "go".to_string(),
        }
    }

    pub fn new_quit_instruction(quit: bool) -> Instruction {
        Instruction {
            body: InstructionKind::QuitInstruct { quit },
            kind: "quit".to_string(),
        }
    }

    pub fn new_sense_instruction(
        sense: bool,
        position: Option<Position>,
        world: PassageMap,
    ) -> Instruction {
        Instruction {
            body: InstructionKind::SenseInstruct {
                sense,
                position,
                world: Box::new(world),
            },
            kind: "sense".to_string(),
        }
    }

    pub fn new_enter_passage_instruction(
        enter: bool,
        passage_key: String,
        map: PassageMap,
    ) -> Instruction {
        Instruction {
            body: InstructionKind::EnterPassageInstruct {
                enter,
                key: passage_key,
                map,
            },

            // TODO parse passage key from natural language
            kind: "enter_passage".to_string(),
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
        // TODO should actually be a world struct with all entity maps
        position: Option<Position>,
        world: Box<PassageMap>,
    },
    EnterPassageInstruct {
        enter: bool,
        key: String,
        map: PassageMap,
    },
}
