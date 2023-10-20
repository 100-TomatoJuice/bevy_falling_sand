use bevy::prelude::{Commands, ResMut, Vec2};

use crate::sandbox::{chunk::SandboxChunk, particle::CollisionType, sandbox::Sandbox};

use super::ColliderStorage;

pub fn ramer_douglas_peucker(data: &[Vec2], epsilon: f32) -> Vec<Vec2> {
    let mut max_distance = 0.0;
    let mut index = 0;
    let end = data.len() - 1;

    for i in 1..end {
        let distance = perpendicular_distance(data[i], data[0], data[end]);
        if distance > max_distance {
            index = i;
            max_distance = distance;
        }
    }

    let mut results = vec![];

    if max_distance > epsilon {
        let mut recursive_results1 = ramer_douglas_peucker(&data[..index], epsilon);
        recursive_results1.remove(recursive_results1.len() - 1);
        let mut recursive_results2 = ramer_douglas_peucker(&data[index..], epsilon);

        // Build result
        results.append(&mut recursive_results1);
        results.append(&mut recursive_results2)
    } else {
        results = vec![data[0], data[end]];
    }

    results
}

// CC0 Tim Sheerman-Chase, 2016
// https://gist.github.com/TimSC/0813573d77734bcb6f2cd2cf6cc7aa51
pub fn perpendicular_distance(point: Vec2, line_start: Vec2, line_end: Vec2) -> f32 {
    let mut dx = line_end.x - line_start.x;
    let mut dy = line_end.y - line_start.y;

    // Normalise
    let magnitude = (dx.powf(2.0) + dy.powf(2.0)).powf(0.5);
    if magnitude > 0.0 {
        dx /= magnitude;
        dy /= magnitude;
    }

    let pvx = point.x - line_start.x;
    let pvy = point.y - line_start.y;

    // Get dot product (project pv onto normalized direction)
    let pvdot = dx * pvx + dy * pvy;

    // Scale line direction vector
    let dsx = pvdot * dx;
    let dsy = pvdot * dy;

    // Subtract this from pv
    let ax = pvx - dsx;
    let ay = pvy - dsy;

    (ax.powf(2.0) + ay.powf(2.0)).powf(0.5)
}

pub fn local_to_world(chunk: &SandboxChunk, local_position: Vec2) -> Vec2 {
    let global_x = local_position.x + (chunk.local_position.0 * chunk.width()) as f32;
    let global_y = local_position.y + (chunk.local_position.1 * chunk.height()) as f32;

    Vec2::new(global_x, global_y)
}

pub fn get_at(sandbox: &Sandbox, x: i32, y: i32, collision_type: CollisionType) -> usize {
    match sandbox.checked_get_i32(x, y) {
        Some(particle) => {
            if particle.collision_type == collision_type {
                1
            } else {
                0
            }
        }
        None => 0,
    }
}

pub fn despawn_old_colliders(
    storage: &mut ResMut<ColliderStorage>,
    i: usize,
    commands: &mut Commands,
) {
    if let Some(colliders) = &storage.colliders[i] {
        for entity in colliders {
            commands.entity(*entity).despawn();
        }
    }
    storage.colliders[i] = None;
}
