use crate::base::space::passage::Passage;

pub struct Instruction {
    pub body: InstructionKind,
}

impl Instruction {
    pub fn new_move_instruction(go: bool, x: i32, y: i32, z: i32) -> Instruction {
        Instruction {
            body: InstructionKind::MoveInstruct,
        }
    }

    pub fn new_quit_instruction(quit: bool) -> Instruction {
        Instruction {
            body: InstructionKind::QuitInstruct,
        }
    }

    pub fn new_sense_instruction(sense: bool) -> Instruction {
        Instruction {
            body: InstructionKind::SenseInstruct,
        }
    }

    pub fn new_enter_passage_instruction(enter: bool, passage_key: String) -> Instruction {
        Instruction {
            body: InstructionKind::EnterPassageInstruct,
        }
    }
}

pub enum InstructionKind {
    MoveInstruct,
    QuitInstruct,
    SenseInstruct,
    EnterPassageInstruct,
}

struct MoveInstruct {
    go: bool,
    key: &'static str,
    x: i32,
    y: i32,
    z: i32,
}

struct SenseInstruct {
    sense: bool,
}

struct EnterPassageInstruct {
    enter: bool,
    passage: Passage,
}

struct QuitInstruct {
    quit: bool,
}
