#[derive(Clone, Copy)]
pub struct Position {
    pub area: Area,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy)]
pub struct Area {
    pub name: &'static str,
    pub zone: Zone,
    pub limits: SpaceLimits,
}

#[derive(Clone, Copy)]
pub struct Zone {
    pub name: &'static str,
    pub is_settlement: bool,
    pub region: Region,
    pub limits: SpaceLimits,
}

#[derive(Clone, Copy)]
pub struct Region {
    pub name: &'static str,
    pub domain: Domain,
    pub limits: SpaceLimits,
}

#[derive(Clone, Copy)]
pub struct Domain {
    pub name: &'static str,
    pub world: World,
    pub limits: SpaceLimits,
}

#[derive(Clone, Copy)]
pub struct World {
    pub name: &'static str,
    pub realm: Realm,
}

#[derive(Clone, Copy)]
pub struct Realm {
    pub name: &'static str,
    pub number: i32,
}

#[derive(Clone, Copy)]
pub struct SpaceLimits {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub min_z: i32,
    pub max_z: i32,
}

impl SpaceLimits {
    pub fn validate(&self, start: Position, x: i32, y: i32, z: i32) -> bool {
        let mut valid = [false, false, false];
        let axes = [x, y, z];

        for (index, distance) in axes.iter().enumerate() {
            let limits = match index {
                0 => (self.min_x, self.max_x),
                1 => (self.min_y, self.max_y),
                2 => (self.min_z, self.max_z),
                _ => unreachable!(),
            };

            let position = match index {
                0 => start.x,
                1 => start.y,
                2 => start.z,
                _ => unreachable!(),
            };

            valid[index] = (position + *distance) > limits.0 && (position + *distance < limits.1)
        }
        valid[0] && valid[1] && valid[2]
    }
}
