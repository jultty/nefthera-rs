use crate::base::space::passage::*;
use crate::lore::locations::zones::*;

pub fn populate() -> PassageMap {
    let mut passages = PassageMap::new();

    fn insert(passages: &mut PassageMap, passage: Passage) {
        passages.insert(passage.key, passage);
    }

    insert(
        &mut passages,
        oppos_outskirts::ibonhaun_lab::get_to_oppos_woods(),
    );
    insert(
        &mut passages,
        oppos_outskirts::oppos_woods::get_to_ibonhaun_lab(),
    );

    passages
}
