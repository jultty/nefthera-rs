use crate::base::space::Area;
use crate::base::space::AreaLimits;
use crate::lore::locations::zones::oppos_outskirts;

pub fn instantiate() -> Area {
    Area {
        name: "Ibonhaun Laboratory",
        zone: oppos_outskirts::instantiate(),
        limits: AreaLimits {
            min_x: -11000,
            max_x: -10000,
            min_y: -71000,
            max_y: -70000,
            max_z: 1000,
            min_z: -1000,
        },
    }
}
