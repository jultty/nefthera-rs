use crate::space::*;

pub fn fiji() -> Domain {
    Domain {
        name: "Fiji",
        world: earth(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: -100000,
            max_y: -80000,
            max_z: 0,
            min_z: 0,
        }
    }
}

fn earth() -> World {
    World {
        name: "Earth",
        realm: leshye(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: -100000,
            max_y: -80000,
            max_z: 0,
            min_z: 0,
        }
    }
}

fn leshye() -> Realm {
    Realm {
        name: "Leshyë",
        number: 0,
    }
}
