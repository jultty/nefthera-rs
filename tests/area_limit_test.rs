use nefthera::demo;
use nefthera::util::instruction::Instruction;

#[test]
fn cant_move_beyond_area_limits() {
    let mut ian = demo::get_demo_character();

    let y_shift = -49;

    let initial_x_position = ian.position.x;
    let initial_y_position = ian.position.y;
    let initial_z_position = ian.position.z;

    // should be able to step just before limit
    let mut go_instruction = Instruction::new_move_instruction(true, 0, y_shift, 0);
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position);

    // should not be able to step on top of limit
    go_instruction = Instruction::new_move_instruction(true, 0, -1, 0);
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position);

    // should not be able to step past limit
    go_instruction = Instruction::new_move_instruction(true, 0, -3, 0);
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position);

    // should not be able to step past x limit either
    go_instruction = Instruction::new_move_instruction(true, 100000, 0, 0);
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position);

    go_instruction = Instruction::new_move_instruction(true, -100000, 0, 0);
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position);

    // should be able to step right before z limit
    go_instruction =
        Instruction::new_move_instruction(true, 0, 0, ian.position.area.limits.max_z - 1);
    ian.go(go_instruction);
    assert_eq!(ian.position.z, ian.position.area.limits.max_z - 1);

    // should not be able to step past x limit either
    go_instruction = Instruction::new_move_instruction(true, -100000, 0, 0);
    ian.go(go_instruction);
    assert_eq!(ian.position.x, initial_x_position);
}
