use crate::space::Position;

impl Character {
    fn r#move(&mut self, x: i32, y: i32, z: i32) -> Position {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
        self.position
    }
}

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
