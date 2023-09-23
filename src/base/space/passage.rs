use super::units::Position;
use std::collections::HashMap;

pub type PassageMap = HashMap<&'static str, Passage>;

#[derive(Clone, Copy)]
pub struct Passage {
    pub key: &'static str,
    open: bool,
    from: Position,
    to: Position,
}

impl Passage {
    pub fn initialize(key: &'static str, open: bool, from: Position, to: Position) -> Passage {
        Passage {
            key,
            open,
            from,
            to,
        }
    }

    pub fn print_key(self) {
        println!(
            "Key: {}, open: {}, from: {}, to: {}",
            self.key, self.open, self.from.area.key, self.to.area.key
        );
    }

    pub fn get_destination(self) -> Option<Position> {
        if self.open {
            Some(self.to)
        } else {
            None
        }
    }
}
