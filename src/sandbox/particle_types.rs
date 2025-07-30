use super::particle::*;
use bevy::utils::default;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ParticleTypes {
    Sand,
    Water,
    Stone,
    Steam,
    Acid,
    Wood,
    Glass,
    Spark,
    Ember,
    Smoke,
    Lava,
    Oil,
    Gunpowder,
    Tnt,
    Ash,
    Dirt,
    Grass,
    Alcohol,
    Igneous,
    Indestructible,
}

pub fn get_particle(particle_type: ParticleTypes) -> Particle {
    match particle_type {
        ParticleTypes::Sand => Particle {
            color: (218, 203, 128, 255),
            density: Density(u32::MAX),
            temperature: Some(Temperature::new(
                50,
                true,
                true,
                false,
                Some(ParticleTypes::Glass),
                0,
            )),
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Water => Particle {
            health: ParticleHealth::new(1, false),
            color: (123, 153, 200, 255),
            movement_type: MovementType::Liquid,
            density: Density(3),
            temperature: Some(Temperature::new(
                30,
                false,
                true,
                false,
                Some(ParticleTypes::Steam),
                0,
            )),
            temperature_changer: Some(TemperatureChanger(5)),
            collision_type: CollisionType::Water,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Stone => Particle {
            color: (125, 110, 110, 255),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            growable_on: true,
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Steam => {
            let tick_health = rand::rng().random_range(100..120);
            Particle {
                health: ParticleHealth::new(tick_health, false),
                color: (240, 233, 201, 255),
                movement_type: MovementType::Gas,
                density: Density(0),
                tick_life: Some(TickLife {
                    replace_on_death: Some(ParticleTypes::Water),
                }),
                affected_by_gravity: true,
                ..default()
            }
        }
        ParticleTypes::Acid => Particle {
            health: ParticleHealth::new(50, false),
            color: (118, 195, 121, 255),
            movement_type: MovementType::Liquid,
            density: Density(4),
            acidity: Some(Acidity(5)),
            collision_type: CollisionType::Acid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Wood => Particle {
            color: (101, 61, 72, 255),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            temperature: Some(Temperature::new(
                30,
                true,
                true,
                false,
                Some(ParticleTypes::Ash),
                0,
            )),
            burnable: Some(Burnable {
                burn_temperature: -1,
                burn_ticks: 50,
                burn_color: (204, 146, 94, 255),
                cooled_color: (125, 110, 110, 255),
                burning: false,
            }),
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Glass => Particle {
            health: ParticleHealth::new(50, false),
            color: (153, 212, 230, 255),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Spark => {
            let tick_health = rand::rng().random_range(5..10);
            Particle {
                health: ParticleHealth::new(tick_health, false),
                color: (204, 146, 94, 255),
                movement_type: MovementType::Gas,
                density: Density(1),
                temperature_changer: Some(TemperatureChanger(-5)),
                tick_life: Some(TickLife {
                    replace_on_death: None,
                }),
                collision_type: CollisionType::Fire,
                affected_by_gravity: true,
                ..default()
            }
        }
        ParticleTypes::Ember => {
            let tick_health = rand::rng().random_range(30..45);
            Particle {
                health: ParticleHealth::new(tick_health, true),
                color: (190, 121, 121, 255),
                density: Density(u32::MAX),
                temperature_changer: Some(TemperatureChanger(-5)),
                tick_life: Some(TickLife {
                    replace_on_death: None,
                }),
                collision_type: CollisionType::Fire,
                affected_by_gravity: true,
                ..default()
            }
        }
        ParticleTypes::Smoke => {
            let tick_health = rand::rng().random_range(40..55);
            Particle {
                health: ParticleHealth::new(tick_health, false),
                color: (36, 22, 41, 255),
                movement_type: MovementType::Gas,
                density: Density(0),
                tick_life: Some(TickLife {
                    replace_on_death: None,
                }),
                affected_by_gravity: true,
                ..default()
            }
        }
        ParticleTypes::Lava => Particle {
            health: ParticleHealth::new(1, false),
            color: (178, 94, 70, 255),
            movement_type: MovementType::Liquid,
            density: Density(5),
            temperature: Some(Temperature::new(
                50,
                true,
                false,
                true,
                Some(ParticleTypes::Igneous),
                0,
            )),
            temperature_changer: Some(TemperatureChanger(-5)),
            collision_type: CollisionType::Fire,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Oil => Particle {
            health: ParticleHealth::new(50, false),
            color: (53, 43, 64, 255),
            movement_type: MovementType::Liquid,
            density: Density(2),
            temperature: Some(Temperature::new(
                5,
                false,
                true,
                false,
                Some(ParticleTypes::Spark),
                0,
            )),
            burnable: Some(Burnable {
                burn_temperature: -2,
                burn_ticks: 15,
                burn_color: (204, 146, 94, 255),
                cooled_color: (125, 110, 110, 255),
                burning: false,
            }),
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Gunpowder => Particle {
            color: (216, 177, 161, 255),
            density: Density(u32::MAX),
            temperature: Some(Temperature::new(1, true, true, false, None, 5)),
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Tnt => Particle {
            color: (147, 63, 69, 255),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            velocity: Velocity::new(0, 0),
            temperature: Some(Temperature::new(1, true, true, false, None, 15)),
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Ash => Particle {
            color: (194, 181, 169, 255),
            density: Density(u32::MAX),
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Dirt => Particle {
            color: (89, 39, 39, 255),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            growable_on: true,
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Grass => Particle {
            color: (80, 141, 118, 255),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            temperature: Some(Temperature::new(1, true, true, false, None, 0)),
            burnable: Some(Burnable {
                burn_temperature: -1,
                burn_ticks: 8,
                burn_color: (204, 146, 94, 255),
                cooled_color: (125, 110, 110, 255),
                burning: false,
            }),
            growable: Some(Growable::new(2, 0.5, 0.25, ParticleTypes::Grass)),
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Alcohol => Particle {
            health: ParticleHealth::new(50, false),
            color: (242, 215, 94, 255),
            movement_type: MovementType::Liquid,
            density: Density(2),
            temperature: Some(Temperature::new(
                5,
                true,
                true,
                false,
                Some(ParticleTypes::Spark),
                0,
            )),
            burnable: Some(Burnable {
                burn_temperature: -2,
                burn_ticks: 15,
                burn_color: (204, 146, 94, 255),
                cooled_color: (125, 110, 110, 255),
                burning: false,
            }),
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Igneous => Particle {
            color: (110, 34, 13, 255),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            collision_type: CollisionType::Solid,
            affected_by_gravity: true,
            ..default()
        },
        ParticleTypes::Indestructible => Particle {
            health: ParticleHealth::new(i32::MAX, false),
            color: (210, 0, 205, 255),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            collision_type: CollisionType::Solid,
            ..default()
        },
    }
}
