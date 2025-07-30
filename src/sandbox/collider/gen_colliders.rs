use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::sandbox::{particle::CollisionType, sandbox::Sandbox};

use super::{utils::*, ColliderStorage};

pub fn generate_sandbox_colliders(
    mut commands: Commands,
    sandbox: Query<&mut Sandbox>,
    mut storage: ResMut<ColliderStorage>,
) {
    if let Ok(sandbox) = sandbox.single() {
        let width = sandbox.width();
        let height = sandbox.height();
    
        let chunks = sandbox.get_all_chunks();
        for (i, chunk) in chunks.iter().enumerate() {
            if !chunk.is_strong_ticked() {
                continue;
            }
            despawn_old_colliders(&mut storage, i, &mut commands);
    
            let low = local_to_world(chunk, Vec2::ZERO);
            let high = local_to_world(
                chunk,
                Vec2::new(chunk.width() as f32, chunk.height() as f32),
            );
    
            let mut colliders = vec![];
            for collision_type in CollisionType::iter() {
                if *collision_type == CollisionType::None {
                    continue;
                }
    
                let blocks = march_edges(sandbox, low, high, *collision_type);
    
                let epsilon = match collision_type {
                    CollisionType::Solid => 1.0,
                    _ => 2.0,
                };
    
                for block in &blocks {
                    let block = ramer_douglas_peucker(block, epsilon);
                    let block = block
                        .into_iter()
                        .map(|pos| {
                            ((pos - Vec2::new(width as f32 / 2.0, height as f32 / 2.0))
                                + Vec2::new(0.5, 0.5))
                                * Vec2::new(8.0, 8.0)
                        })
                        .collect();
    
                    let collider = match collision_type {
                        CollisionType::None => panic!(),
                        CollisionType::Solid => commands.spawn(Collider::polyline(block, None)),
                        CollisionType::Acid => commands.spawn((
                            Collider::polyline(block, None),
                            Sensor,
                            // Acid Status Marker
                        )),
                        CollisionType::Fire => commands.spawn((
                            Collider::polyline(block, None),
                            Sensor,
                            // Fire Status Marker
                        )),
                        CollisionType::Water => commands.spawn((
                            Collider::polyline(block, None),
                            Sensor,
                            // Water Status Marker
                        )),
                    }
                    .id();
                    colliders.push(collider);
                }
            }
    
            storage.colliders[i] = Some(colliders);
        }
    }
}

// Based on https://github.com/shnewto/bevy_rapier_collider_gen/blob/main/src/edge.rs#L56
pub fn march_edges(
    sandbox: &Sandbox,
    low: Vec2,
    high: Vec2,
    collision_type: CollisionType,
) -> Vec<Vec<Vec2>> {
    let mut edge_points: Vec<Vec2> = vec![];

    for x in low.x as i32..=high.x as i32 {
        for y in low.y as i32..=high.y as i32 {
            if get_at(sandbox, x, y, collision_type) == 0 {
                continue;
            }

            let neighbors = [
                get_at(sandbox, x + 1, y, collision_type),
                get_at(sandbox, x - 1, y, collision_type),
                get_at(sandbox, x, y + 1, collision_type),
                get_at(sandbox, x, y - 1, collision_type),
            ];

            let (x, y) = (x as f32, y as f32);
            match neighbors {
                // Corners
                [1, 0, 0, 1] => {
                    edge_points.push(Vec2::new(x - 0.5, y - 0.5));
                    edge_points.push(Vec2::new(x - 0.5, y + 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y + 0.5));
                }
                [1, 0, 1, 0] => {
                    edge_points.push(Vec2::new(x - 0.5, y + 0.5));
                    edge_points.push(Vec2::new(x - 0.5, y - 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y - 0.5));
                }
                [0, 1, 0, 1] => {
                    edge_points.push(Vec2::new(x - 0.5, y + 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y + 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y - 0.5));
                }
                [0, 1, 1, 0] => {
                    edge_points.push(Vec2::new(x + 0.5, y + 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y - 0.5));
                    edge_points.push(Vec2::new(x - 0.5, y - 0.5));
                }
                // Sides
                [1, 1, 1, 0] | [0, 0, 1, 0] => {
                    edge_points.push(Vec2::new(x - 0.5, y - 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y - 0.5));
                }
                [1, 1, 0, 1] | [0, 0, 0, 1] => {
                    edge_points.push(Vec2::new(x - 0.5, y + 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y + 0.5));
                }
                [1, 0, 1, 1] | [1, 0, 0, 0] => {
                    edge_points.push(Vec2::new(x - 0.5, y - 0.5));
                    edge_points.push(Vec2::new(x - 0.5, y + 0.5));
                }
                [0, 1, 1, 1] | [0, 1, 0, 0] => {
                    edge_points.push(Vec2::new(x + 0.5, y - 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y + 0.5));
                }
                // Surrounded
                [1, 1, 1, 1] => continue,
                // Others
                _ => {
                    edge_points.push(Vec2::new(x + 0.5, y + 0.5));
                    edge_points.push(Vec2::new(x - 0.5, y + 0.5));
                    edge_points.push(Vec2::new(x - 0.5, y - 0.5));
                    edge_points.push(Vec2::new(x + 0.5, y - 0.5));
                }
            }
        }
    }

    points_to_drawing_order(&edge_points)
}

fn points_to_drawing_order(points: &[Vec2]) -> Vec<Vec<Vec2>> {
    let mut edge_points: Vec<Vec2> = points.to_vec();
    let mut in_drawing_order: Vec<Vec2> = vec![];
    let mut groups: Vec<Vec<Vec2>> = vec![];
    while !edge_points.is_empty() {
        if in_drawing_order.is_empty() {
            in_drawing_order.push(edge_points.remove(0));
        }

        let prev = *in_drawing_order.last().unwrap();

        let neighbor = edge_points
            .iter()
            .enumerate()
            .find(|(_, p)| prev.distance(**p) == 1.0);

        if let Some((i, _)) = neighbor {
            let next = edge_points.remove(i);
            in_drawing_order.push(next);
            continue;
        }

        if !in_drawing_order.is_empty() {
            groups.push(in_drawing_order.clone());
            in_drawing_order.clear()
        }
    }

    if !in_drawing_order.is_empty() {
        groups.push(in_drawing_order.clone());
    }

    groups
}
