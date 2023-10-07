use nefthera::demo;
use nefthera::util::instruction::Instruction;

#[test]
fn ibonhaun_out_passage_is_mapped() {
    let mut ian = demo::get_demo_character();
    let entities = demo::get_entity_map().clone();

    let mut position = ian.go(Instruction::new_move_instruction(true, 0, -9, 0, &entities));
    let mut present_entities = entities.get(&position);
    assert!(present_entities.is_none());

    position = ian.go(Instruction::new_move_instruction(
        true, 0, -40, 0, &entities,
    ));
    present_entities = entities.get(&position);
    assert_eq!(
        present_entities.unwrap().passages.entities[0].key,
        "IBONHAUN-LAB_TO_OPPOS-WOODS"
    );
}

#[test]
fn ibonhaun_out_passage_can_be_sensed() {
    let mut ian = demo::get_demo_character();
    let entities = demo::get_entity_map().clone();

    ian.go(Instruction::new_move_instruction(
        true, 0, -49, 0, &entities,
    ));
    let sensed_entities = ian.sense(Instruction::new_sense_instruction(
        true,
        Some(ian.position),
        &entities,
    ));
    assert_eq!(
        sensed_entities.passages.entities[0].key,
        "IBONHAUN-LAB_TO_OPPOS-WOODS"
    );
}

#[test]
fn exactly_one_passage_can_be_sensed_at_ibonhaun_out() {
    let mut ian = demo::get_demo_character();
    let entities = demo::get_entity_map().clone();

    ian.go(Instruction::new_move_instruction(
        true, 0, -49, 0, &entities,
    ));
    let sensed_passages = ian
        .sense(Instruction::new_sense_instruction(
            true,
            Some(ian.position),
            &entities,
        ))
        .passages
        .entities;
    assert_eq!(sensed_passages.len(), 1)
}
