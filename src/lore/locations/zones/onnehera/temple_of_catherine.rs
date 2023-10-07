use crate::base::description::Description;
use crate::base::space::limit::SpaceLimits;
use crate::base::space::units::Area;
use crate::lore::locations::zones::onnehera;

pub fn instantiate() -> Area {
    Area {
        name: "Temple of Catherine",
        key: "ONNEHERA_TEMPLE-OF-CATHERINE",
        zone: onnehera::instantiate(),
        description: Description::new("Dedicated to the Goddess of the Night, the temple received very few visitors during the day."),
        limits: SpaceLimits {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            min_z: 0,
        },
    }
}
