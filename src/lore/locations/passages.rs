use crate::base::space::entity::*;
use crate::base::space::passage::*;
use crate::lore::locations::zones::*;

pub fn populate() -> EntityGroup<Passage> {
    let mut passages = EntityGroup {
        entities: Vec::<Passage>::new(),
    };

    fn insert(group: &mut EntityGroup<Passage>, passage: Passage) {
        group.entities.push(passage);
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
