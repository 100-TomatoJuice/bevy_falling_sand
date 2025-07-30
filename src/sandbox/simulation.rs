use bevy::prelude::*;

use super::effects::acidity::tick_acidity;
use super::effects::growable::tick_growable;
use super::effects::movement::tick_movement;
use super::effects::temperature::tick_temperature;
use super::effects::tick_life::tick_life;
use super::sandbox::*;

pub fn update_particles(mut sandbox_query: Query<&mut Sandbox>) {
    let mut sandbox = sandbox_query
        .single_mut()
        .expect("There should be a Sandbox at this point");

    sandbox.reset_ticked_chunks();

    for x in 0..sandbox.width() {
        for y in 0..sandbox.height() {
            let current_chunk = sandbox.get_chunk(x, y);
            if !current_chunk.is_weak_ticked() && !current_chunk.is_strong_ticked() {
                continue;
            }

            step_particle(x, y, &mut sandbox);
        }
    }

    sandbox.reset_updated();
}

fn step_particle(x: usize, y: usize, sandbox: &mut Sandbox) {
    match sandbox.get(x, y) {
        Some(particle) => {
            if particle.updated {
                return;
            }
            if particle.health.amount <= 0 {
                sandbox.set(x, y, None);
                return;
            }
        }
        None => return,
    }

    if tick_acidity(x, y, sandbox) {
        return;
    }
    if tick_temperature(x, y, sandbox) {
        return;
    }
    if tick_life(x, y, sandbox) {
        return;
    }

    tick_growable(x, y, sandbox);
    tick_movement(x, y, sandbox);
}
