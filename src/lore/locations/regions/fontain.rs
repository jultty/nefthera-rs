use crate::space::*;
use crate::lore::locations::domains::fiji;

pub fn fontain() -> Region {
    Region {
        name: "Fontain",
        domain: fiji(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: -100000,
            max_y: -80000,
            max_z: 0,
            min_z: 0,
        },
    }
}
