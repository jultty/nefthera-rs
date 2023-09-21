use crate::base::space::*;

pub fn get_fiji() -> Domain {
    Domain {
        name: "Fiji",
        world: get_earth(),
        limits: AreaLimits {
            min_x: 0,
            max_x: 0,
            min_y: -100000,
            max_y: -80000,
            max_z: 0,
            min_z: 0,
        },
    }
}

fn get_earth() -> World {
    World {
        name: "Earth",
        realm: get_leshye(),
        limits: AreaLimits {
            min_x: 0,
            max_x: 0,
            min_y: -100000,
            max_y: -80000,
            max_z: 0,
            min_z: 0,
        },
    }
}

fn get_leshye() -> Realm {
    Realm {
        name: "Leshyë",
        number: 0,
    }
}
