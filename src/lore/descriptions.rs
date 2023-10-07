use crate::base::description::Description;
use crate::base::space::units::Position;
use crate::lore::locations::zones;

pub fn get_vector() -> Vec<(Position, Description)> {
    vec![
        (
            Position {
                area: zones::oppos_outskirts::ibonhaun_lab::instantiate(),
                x: -10500,
                y: -70500,
                z: 0,
            },
            Description::new(
                "A table on this spot had its surface all scratched from working with sharp tools.",
            ),
        ),
        (
            Position {
                area: zones::oppos_outskirts::ibonhaun_lab::instantiate(),
                x: -10499,
                y: -70500,
                z: 0,
            },
            Description::new(
                "Alexander's chair, which seemed very comfortable and worn, lied at this spot.",
            ),
        ),
    ]
}
