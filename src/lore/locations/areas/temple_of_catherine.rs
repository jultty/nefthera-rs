use crate::lore::locations::zones::onnehera::onnehera;
use crate::space::Area;
use crate::space::AreaRange;

pub fn temple_of_catherine() -> Area {
    Area {
        name: "Temple of Catherine",
        zone: onnehera(),
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
