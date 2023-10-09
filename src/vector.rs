use bevy::prelude::Vec2;

pub fn rotate_45_clockwise(x: i32, y: i32) -> (i32, i32) {
    let vector = Vec2::new(x as f32, y as f32);
    let magnitude = vector.abs().max_element() as i32;

    let vector = vector.normalize();
    let vector = (vector.x as i32, vector.y as i32);

    match vector {
        (0, 1) => (magnitude, magnitude),
        (1, 1) => (magnitude, 0),
        (1, 0) => (magnitude, -magnitude),
        (1, -1) => (0, -magnitude),
        (0, -1) => (-magnitude, -magnitude),
        (-1, -1) => (-magnitude, 0),
        (-1, 0) => (-magnitude, magnitude),
        (-1, 1) => (0, magnitude),
        (0, 0) => (0, 0),
        _ => panic!("The vector was not normalized in a 8-way integar"),
    }
}

pub fn rotate_45_counterclockwise(x: i32, y: i32) -> (i32, i32) {
    let vector = Vec2::new(x as f32, y as f32);
    let magnitude = vector.abs().max_element() as i32;

    let vector = vector.normalize();
    let vector = (vector.x as i32, vector.y as i32);

    match vector {
        (0, 1) => (-magnitude, magnitude),
        (-1, 1) => (-magnitude, 0),
        (-1, 0) => (-magnitude, -magnitude),
        (-1, -1) => (0, -magnitude),
        (0, -1) => (magnitude, -magnitude),
        (1, -1) => (magnitude, 0),
        (1, 0) => (magnitude, magnitude),
        (1, 1) => (0, magnitude),
        (0, 0) => (0, 0),
        _ => panic!("The vector was not normalized in a 8-way integar"),
    }
}

pub fn rotate_90_clockwise(x: i32, y: i32) -> (i32, i32) {
    let vector = Vec2::new(x as f32, y as f32);
    let magnitude = vector.abs().max_element() as i32;

    let vector = vector.normalize();
    let vector = (vector.x as i32, vector.y as i32);

    (vector.1 * magnitude, -vector.0 * magnitude)
}

pub fn rotate_90_counterclockwise(x: i32, y: i32) -> (i32, i32) {
    let vector = Vec2::new(x as f32, y as f32);
    let magnitude = vector.abs().max_element() as i32;

    let vector = vector.normalize();
    let vector = (vector.x as i32, vector.y as i32);

    (-vector.1 * magnitude, vector.0 * magnitude)
}

pub fn rotate_90_clockwise_normalized(x: i32, y: i32) -> (i32, i32) {
    let vector = Vec2::new(x as f32, y as f32);
    let vector = vector.normalize();
    let vector = (vector.x as i32, vector.y as i32);

    (vector.1, -vector.0)
}

pub fn rotate_90_counterclockwise_normalized(x: i32, y: i32) -> (i32, i32) {
    let vector = Vec2::new(x as f32, y as f32);
    let vector = vector.normalize();
    let vector = (vector.x as i32, vector.y as i32);

    (-vector.1, vector.0)
}
