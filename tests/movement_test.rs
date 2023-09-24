use nefthera::demo;
use nefthera::util::instruction::Instruction;

#[test]
fn can_move() {
    let mut ian = demo::get_demo_character();
    let world = demo::get_passage_map();

    let x_shift = 6;
    let y_shift = 3;
    let z_shift = 2;

    let initial_x_position = ian.position.x;
    let initial_y_position = ian.position.y;
    let initial_z_position = ian.position.z;

    let go_instruction =
        Instruction::new_move_instruction(true, world.clone(), x_shift, y_shift, z_shift);
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position + x_shift);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position + z_shift);
}

#[test]
fn can_move_backwards() {
    let mut ian = demo::get_demo_character();
    let world = demo::get_passage_map();

    let x_shift = 6;
    let y_shift = 3;
    let z_shift = 9;

    let initial_x_position = ian.position.x;
    let initial_y_position = ian.position.y;
    let initial_z_position = ian.position.z;

    let mut go_instruction =
        Instruction::new_move_instruction(true, world.clone(), x_shift, y_shift, z_shift);
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position + x_shift);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position + z_shift);

    go_instruction = Instruction::new_move_instruction(
        true,
        world.clone(),
        x_shift * -1,
        y_shift * -1,
        z_shift * -1,
    );
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position);
    assert_eq!(ian.position.y, initial_y_position);
    assert_eq!(ian.position.z, initial_z_position);
}
