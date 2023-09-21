use crate::base::space::Area;
use crate::base::space::AreaRange;
use crate::lore::locations::zones::oppos_outskirts;

pub fn instantiate() -> Area {
    Area {
        name: "Ibonhaun Laboratory",
        zone: oppos_outskirts::instantiate(),
        range: AreaRange {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
        },
    }
}
