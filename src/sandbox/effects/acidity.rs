use crate::sandbox::sandbox::Sandbox;

/// Returns true if the current particle was removed from the simulation during the tick
pub fn tick_acidity(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    let acidity = match sandbox
        .get(x, y)
        .expect("Simulation shouldn't have let it get this far")
        .acidity
    {
        Some(acid) => acid.0,
        None => return false,
    };
    if acidity <= 0 {
        return false;
    }

    // Tracks how many times acid as affect pther particles
    let mut acid_ticks = 0;

    for (neighbor_x, neighbor_y) in [
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
        (x, y.overflowing_sub(1).0),
        (x, y + 1),
    ] {
        if let Some(particle) = sandbox.checked_get_mut(neighbor_x, neighbor_y) {
            let health = &mut particle.health;
            if !health.corrodable {
                continue;
            }

            health.amount -= acidity;
            acid_ticks += 1;

            if health.amount <= 0 {
                sandbox.set(neighbor_x, neighbor_y, None);
            }

            sandbox.get_chunk_mut(neighbor_x, neighbor_y).weak_tick();
        }
    }

    sandbox.get_chunk_mut(x, y).weak_tick();

    let acid_health = &mut sandbox.get_mut(x, y).unwrap().health;
    acid_health.amount -= acid_ticks;

    if acid_health.amount <= 0 {
        sandbox.set(x, y, None);
        return true;
    }

    false
}
