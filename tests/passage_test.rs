use nefthera::demo;
use nefthera::util::instruction::Instruction;

#[test]
fn passages_between_ibonhaun_lab_and_oppos_woods_work() {
    let mut ian = demo::get_demo_character();
    let entities = demo::get_entity_map().clone();

    assert_eq!(ian.position.area.name, "Ibonhaun Laboratory");

    ian.go(Instruction::new_move_instruction(
        true, 0, -49, 0, &entities,
    ));
    let mut sensed_passages = ian
        .sense(Instruction::new_sense_instruction(
            true,
            Some(ian.position),
            &entities,
        ))
        .passages
        .entities;
    assert_eq!(sensed_passages.len(), 1);

    ian.enter_passage(Instruction::new_enter_passage_instruction(
        true,
        sensed_passages[0].key.to_string(),
        &entities,
    ));

    assert_eq!(ian.position.area.name, "Oppos Woods");

    sensed_passages = ian
        .sense(Instruction::new_sense_instruction(
            true,
            Some(ian.position),
            &entities,
        ))
        .passages
        .entities;
    assert_eq!(sensed_passages.len(), 1);

    ian.enter_passage(Instruction::new_enter_passage_instruction(
        true,
        sensed_passages[0].key.to_string(),
        &entities,
    ));

    assert_eq!(ian.position.area.name, "Ibonhaun Laboratory");
}
