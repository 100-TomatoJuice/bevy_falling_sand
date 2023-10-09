use crate::sandbox::{particle_types::get_particle, sandbox::Sandbox};

/// Returns true if the current particle was removed from the simulation during the tick
pub fn tick_life(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    let replacement = match sandbox
        .get(x, y)
        .expect("Simulation shouldn't have let it get this far")
        .tick_life
    {
        Some(life) => life.replace_on_death,
        None => return false,
    };

    let health = &mut sandbox
        .get_mut(x, y)
        .expect("Simulation shouldn't have let it get this far")
        .health;
    health.amount -= 1;

    if health.amount <= 0 {
        let replacement = replacement.map(get_particle);

        sandbox.set(x, y, replacement);
        return true;
    }

    sandbox.get_chunk_mut(x, y).weak_tick();

    false
}
