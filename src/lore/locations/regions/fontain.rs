use crate::base::space::*;
use crate::lore::locations::domains;

pub fn instantiate() -> Region {
    Region {
        name: "Fontain",
        domain: domains::get_fiji(),
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
