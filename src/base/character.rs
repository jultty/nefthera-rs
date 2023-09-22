use crate::base::space::Position;

impl Character {
    pub fn go(&mut self, x: i32, y: i32, z: i32) -> Position {
        let axes = [x, y, z];

        for (index, index_distance) in axes.iter().enumerate() {
            let distance = *index_distance;

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

        self.position
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
