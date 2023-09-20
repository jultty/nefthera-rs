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
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy)]
pub struct Map {
    pub name: &'static str,
    pub world: World,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy)]
pub struct World {
    pub name: &'static str,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
