#[derive(Clone, Copy)]
pub struct Position {
    area: Area,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy)]
pub struct Area {
    map: Map,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy)]
pub struct Map {
    world: World,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy)]
pub struct World {
    x: i32,
    y: i32,
    z: i32,
}
