pub struct Position {
    area: Area,
    x: i32,
    y: i32,
    z: i32,
}

pub struct Area {
    map: Map,
    x: i32,
    y: i32,
    z: i32,
}

pub struct Map {
    world: World,
    x: i32,
    y: i32,
    z: i32,
}

pub struct World {
    x: i32,
    y: i32,
    z: i32,
}
