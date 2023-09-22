use crate::base::space::limit::SpaceLimits;
use crate::base::space::units::{Realm, World, Domain};

pub fn get_fiji() -> Domain {
    Domain {
        name: "Fiji",
        world: get_earth(),
        limits: SpaceLimits {
            min_x: -100000,
            max_x: 100000,
            min_y: -100000,
            max_y: 100000,
            max_z: 1000,
            min_z: -1000,
        },
    }
}

fn get_earth() -> World {
    World {
        name: "Earth",
        realm: get_leshye(),
    }
}

fn get_leshye() -> Realm {
    Realm {
        name: "Leshyë",
        number: 0,
    }
}
