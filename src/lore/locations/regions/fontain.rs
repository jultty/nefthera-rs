use crate::base::space::limit::SpaceLimits;
use crate::base::space::units::Region;
use crate::lore::locations::domains;

pub fn instantiate() -> Region {
    Region {
        name: "East Fontain",
        domain: domains::get_fiji(),
        limits: SpaceLimits {
            min_x: -25000,
            max_x: 0,
            min_y: -75000,
            max_y: -50000,
            max_z: 1000,
            min_z: 1000,
        },
    }
}
