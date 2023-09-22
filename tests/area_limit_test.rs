use nefthera::demo;

#[test]
fn characters_cant_move_beyond_area_limits() {
    let mut ian = demo::get_demo_character();

    let y_shift = -49;

    let initial_x_position = ian.position.x;
    let initial_y_position = ian.position.y;
    let initial_z_position = ian.position.z;

    // should be able to step just before limit
    ian.go(0, y_shift, 0);
    assert_eq!(ian.position.x, initial_x_position);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position);

    // should not be able to step on top of limit
    ian.go(0, -1, 0);
    assert_eq!(ian.position.x, initial_x_position);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position);

    // should not be able to step past limit
    ian.go(0, -3, 0);
    assert_eq!(ian.position.x, initial_x_position);
    assert_eq!(ian.position.y, initial_y_position + y_shift);
    assert_eq!(ian.position.z, initial_z_position);

    // should not be able to step past x limit either
    ian.go(100000, 0, 0);
    assert_eq!(ian.position.x, initial_x_position);

    ian.go(-100000, 0, 0);
    assert_eq!(ian.position.x, initial_x_position);

    // should be able to step right before z limit
    ian.go(0, 0, ian.position.area.limits.max_z - 1);
    assert_eq!(ian.position.z, ian.position.area.limits.max_z - 1);

    // should not be able to step past x limit either
    ian.go(-100000, 0, 0);
    assert_eq!(ian.position.x, initial_x_position);
}
