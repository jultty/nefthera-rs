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
    pub limits: AreaLimits,
}

#[derive(Clone, Copy)]
pub struct Zone {
    pub name: &'static str,
    pub is_settlement: bool,
    pub region: Region,
    pub limits: AreaLimits,
}

#[derive(Clone, Copy)]
pub struct Region {
    pub name: &'static str,
    pub domain: Domain,
    pub limits: AreaLimits,
}

#[derive(Clone, Copy)]
pub struct Domain {
    pub name: &'static str,
    pub world: World,
    pub limits: AreaLimits,
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
pub struct AreaLimits {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub min_z: i32,
    pub max_z: i32,
}
