use crate::space::Area;
use crate::space::AreaRange;
use crate::lore::locations::zones::oppos_outskirts::oppos_outskirts;

pub fn ibonhaun_lab() -> Area {
    Area {
        name: "Ibonhaun Laboratory",
        zone: oppos_outskirts(),
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

