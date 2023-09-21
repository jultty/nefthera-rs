use crate::character::Character;
use std::collections::HashSet;

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
    pub map: Map,
    pub range: AreaRange,
}

#[derive(Clone, Copy)]
pub struct Map {
    pub name: &'static str,
    pub world: World,
    pub range: AreaRange,
}

#[derive(Clone, Copy)]
pub struct World {
    pub name: &'static str,
    pub range: AreaRange,
}

#[derive(Clone, Copy)]
pub struct AreaRange {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub min_z: i32,
    pub max_z: i32,
}
