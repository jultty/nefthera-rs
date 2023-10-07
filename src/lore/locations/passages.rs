use crate::base::space::passage::*;
use crate::lore::locations::zones::*;

pub fn get_vector() -> Vec<Passage> {
    vec![
        oppos_outskirts::ibonhaun_lab::get_to_oppos_woods(),
        oppos_outskirts::oppos_woods::get_to_ibonhaun_lab(),
    ]
}
