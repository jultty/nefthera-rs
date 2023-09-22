use nefthera::demo;

#[test]
fn characters_can_move() {
    let mut ian = demo::get_demo_character();

    let x_shift = 6;
    let y_shift = 3;
    let z_shift = 2;

    let initial_x_position = ian.position.x;
    let initial_y_position = ian.position.y;
    let initial_z_position = ian.position.z;

    ian.go(x_shift, y_shift, z_shift);
    assert_eq!(ian.position.x, initial_x_position + x_shift);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position + z_shift);
}

#[test]
fn characters_can_move_backwards() {
    let mut ian = demo::get_demo_character();

    let x_shift = 6;
    let y_shift = 3;
    let z_shift = 9;

    let initial_x_position = ian.position.x;
    let initial_y_position = ian.position.y;
    let initial_z_position = ian.position.z;

    ian.go(x_shift, y_shift, z_shift);
    assert_eq!(ian.position.x, initial_x_position + x_shift);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position + z_shift);

    ian.go(x_shift * -1, y_shift * -1, z_shift * -1);
    assert_eq!(ian.position.x, initial_x_position);
    assert_eq!(ian.position.y, initial_y_position);
    assert_eq!(ian.position.z, initial_z_position);
}
