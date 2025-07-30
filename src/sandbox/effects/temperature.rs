use bevy::prelude::Vec2;
use rand::{rng, Rng};

use crate::sandbox::{particle::*, particle_types::*, sandbox::Sandbox};

/// Returns true if the current particle was removed from the simulation during the tick
pub fn tick_temperature(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    temperature_change_neighbors(x, y, sandbox);

    if tick_self(x, y, sandbox) {
        return true;
    }

    try_ignite_burnable(x, y, sandbox);
    try_extinquish_burning(x, y, sandbox);
    spark_if_ignited(x, y, sandbox);

    false
}

fn temperature_change_neighbors(x: usize, y: usize, sandbox: &mut Sandbox) {
    let temperature_changer = match sandbox
        .get(x, y)
        .expect("Simulation shouldn't have let it get this far")
        .temperature_changer
    {
        Some(changer) => changer.0,
        None => return,
    };

    for (neighbor_x, neighbor_y) in [
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
        (x, y.overflowing_sub(1).0),
        (x, y + 1),
    ] {
        if let Some(particle) = sandbox.checked_get_mut(neighbor_x, neighbor_y) {
            if let Some(temperature) = &mut particle.temperature {
                if temperature_changer.is_positive() && !temperature.coolable {
                    continue;
                }
                if temperature_changer.is_negative() && !temperature.heatable {
                    continue;
                }

                match temperature.critical_on_cool {
                    true => {
                        temperature.current_temperature =
                            (temperature.current_temperature + temperature_changer).clamp(1, 100)
                    }
                    false => {
                        temperature.current_temperature = (temperature.current_temperature
                            + temperature_changer)
                            .clamp(0, temperature.starting_temperature)
                    }
                }
            }
        }
    }
}

fn tick_self(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    let temperature = match sandbox
        .get(x, y)
        .expect("Simulation shouldn't have let it get this far")
        .temperature
    {
        Some(temp) => temp,
        None => return false,
    };

    let health = &mut sandbox.get_mut(x, y).unwrap().health;

    if (!temperature.critical_on_cool && temperature.current_temperature <= 0)
        || (temperature.critical_on_cool && temperature.current_temperature >= 100)
    {
        if temperature.explosion_radius > 0 {
            explode(x, y, temperature.explosion_radius, sandbox);
            return true;
        }

        deplete_critical(health);

        if health.amount <= 0 {
            let replacement = temperature.change_on_critical.map(get_particle);

            sandbox.set(x, y, replacement);
            return true;
        }

        sandbox.get_chunk_mut(x, y).weak_tick();
    }

    false
}

fn try_ignite_burnable(x: usize, y: usize, sandbox: &mut Sandbox) {
    let particle = sandbox
        .get_mut(x, y)
        .expect("Simulation shouldn't have let it get this far");
    if let Some(burnable) = &mut particle.burnable {
        if burnable.burning || particle.temperature.unwrap().current_temperature > 0 {
            return;
        }

        burnable.burning = true;
        particle.temperature_changer = Some(TemperatureChanger(-1));
        particle.health.amount = burnable.burn_ticks;
        particle.color = burnable.burn_color;
    }
}

fn try_extinquish_burning(x: usize, y: usize, sandbox: &mut Sandbox) {
    let particle = sandbox
        .get_mut(x, y)
        .expect("Simulation shouldn't have let it get this far");
    if let Some(burnable) = &mut particle.burnable {
        if !burnable.burning || particle.temperature.unwrap().current_temperature <= 0 {
            return;
        }

        burnable.burning = false;
        particle.temperature_changer = None;
        particle.health.amount = burnable.burn_ticks;
        particle.color = burnable.cooled_color;
        particle.temperature.unwrap().current_temperature =
            particle.temperature.unwrap().starting_temperature;
    }
}

fn spark_if_ignited(x: usize, y: usize, sandbox: &mut Sandbox) {
    match sandbox
        .get_mut(x, y)
        .expect("Simulation shouldn't have let it get this far")
        .burnable
    {
        Some(burnable) => {
            if !burnable.burning {
                return;
            }
        }
        None => return,
    };

    for (neighbor_x, neighbor_y) in [
        (x, y + 1),
        (x + 1, y),
        (x.overflowing_sub(1).0, y),
        (x, y.overflowing_sub(1).0),
    ] {
        if sandbox.checked_get(neighbor_x, neighbor_y).is_none()
            && !sandbox.out_of_bounds_usize(neighbor_x, neighbor_y)
        {
            let new_particle = if rng().random_ratio(1, 3) {
                get_particle(ParticleTypes::Spark)
            } else {
                get_particle(ParticleTypes::Smoke)
            };

            sandbox.set(neighbor_x, neighbor_y, Some(new_particle));
        }
    }
}

fn explode(current_x: usize, current_y: usize, radius: i32, sandbox: &mut Sandbox) {
    let low_x = current_x as i32 - radius;
    let high_x = current_x as i32 + radius;
    let low_y = current_y as i32 - radius;
    let high_y = current_y as i32 + radius;

    for x in (low_x - radius)..=(high_x + radius) {
        for y in (low_y - radius)..=(high_y + radius) {
            if sandbox.out_of_bounds_i32(x, y) {
                continue;
            }

            if let Some(particle) = sandbox.get_mut(x as usize, y as usize) {
                if x < low_x || x > high_x || y < low_y || y > high_y {
                    let force = (Vec2::new(x as f32, y as f32)
                        - Vec2::new(current_x as f32, current_y as f32))
                    .normalize()
                        * 10.0;
                    particle.velocity = Velocity::new(force.x as i32, force.y.abs() as i32);

                    continue;
                }

                sandbox.set(
                    x as usize,
                    y as usize,
                    Some(get_particle(ParticleTypes::Spark)),
                );
            }
        }
    }
}

fn deplete_critical(health: &mut ParticleHealth) {
    health.amount -= 1;
}
