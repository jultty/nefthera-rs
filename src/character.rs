use crate::space::Position;

pub struct Character {
    pub name: String,
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
