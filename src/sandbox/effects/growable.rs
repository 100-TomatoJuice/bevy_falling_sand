use rand::prelude::*;

use crate::sandbox::{particle_types::get_particle, sandbox::Sandbox};

pub fn tick_growable(x: usize, y: usize, sandbox: &mut Sandbox) {
    sandbox.get_chunk_mut(x, y).weak_tick();

    if try_spread(x, y, sandbox) {
        return;
    }

    try_upwards_growth(x, y, sandbox);
}

fn try_spread(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    let grow_as = match sandbox
        .get(x, y)
        .expect("Simulation shouldn't have let it get this far")
        .growable
    {
        Some(growable) => {
            if !rand::rng().random_bool(growable.spread_chance) {
                return false;
            }

            growable.grow_as
        }
        None => return false,
    };

    let mut search_directions = [
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
        (x, y.overflowing_sub(1).0),
        (x, y + 1),
    ];
    search_directions.shuffle(&mut rand::rng());

    for (neighbor_x, neighbor_y) in search_directions {
        if let Some(particle) = sandbox.checked_get(neighbor_x, neighbor_y) {
            if !particle.growable_on || sandbox.eight_surrounded(neighbor_x, neighbor_y) {
                continue;
            }

            let mut new_particle = get_particle(grow_as);
            new_particle.updated = true;
            sandbox.set(neighbor_x, neighbor_y, Some(new_particle));
            return true;
        }
    }

    false
}

fn try_upwards_growth(x: usize, y: usize, sandbox: &mut Sandbox) {
    let growable = match sandbox
        .get(x, y)
        .expect("Simulation shouldn't have let it get this far")
        .growable
    {
        Some(growable) => {
            if !rand::rng().random_bool(growable.spread_chance) || !growable.can_sprout {
                return;
            }

            growable
        }
        None => return,
    };

    let mut search_directions = [
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
        (x, y + 1),
        (x, y.overflowing_sub(1).0),
    ];
    search_directions.shuffle(&mut rand::rng());

    for (neighbor_x, neighbor_y) in search_directions {
        if sandbox.checked_get(neighbor_x, neighbor_y).is_some()
            || sandbox.out_of_bounds_usize(neighbor_x, neighbor_y)
        {
            continue;
        }

        let mut new_particle = get_particle(growable.grow_as);
        new_particle.updated = true;
        sandbox.set(neighbor_x, neighbor_y, Some(new_particle));
    }
}
