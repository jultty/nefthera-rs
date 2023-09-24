use super::units::Position;
use std::collections::HashMap;

pub type PassageMap = HashMap<Position, Vec<Passage>>;

#[derive(Clone, Copy)]
pub struct Passage {
    pub name: &'static str,
    pub key: &'static str,
    open: bool,
    from: Position,
    to: Position,
}

impl Passage {
    pub fn initialize(
        name: &'static str,
        key: &'static str,
        open: bool,
        from: Position,
        to: Position,
    ) -> Passage {
        Passage {
            name,
            key,
            open,
            from,
            to,
        }
    }

    pub fn get_destination(self) -> Option<Position> {
        if self.open {
            Some(self.to)
        } else {
            None
        }
    }

    pub fn get_origin(self) -> Option<Position> {
        if self.open {
            Some(self.from)
        } else {
            None
        }
    }
}
