use super::limit::SpaceLimits;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub area: Area,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Area {
    pub name: &'static str,
    pub key: &'static str,
    pub zone: Zone,
    pub limits: SpaceLimits,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Zone {
    pub name: &'static str,
    pub is_settlement: bool,
    pub region: Region,
    pub limits: SpaceLimits,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Region {
    pub name: &'static str,
    pub domain: Domain,
    pub limits: SpaceLimits,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Domain {
    pub name: &'static str,
    pub world: World,
    pub limits: SpaceLimits,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct World {
    pub name: &'static str,
    pub realm: Realm,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Realm {
    pub name: &'static str,
    pub number: i32,
}
