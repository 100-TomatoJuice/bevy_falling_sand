use std::cmp::Ordering;
use std::slice::Iter;

use rand::prelude::*;

use super::particle_types::ParticleTypes;

#[derive(Clone, Copy, Default)]
pub struct Particle {
    pub health: ParticleHealth,
    pub velocity: Velocity,
    pub color: (u8, u8, u8, u8),
    pub movement_type: MovementType,
    pub density: Density,
    pub acidity: Option<Acidity>,
    pub temperature: Option<Temperature>,
    pub temperature_changer: Option<TemperatureChanger>,
    pub burnable: Option<Burnable>,
    pub tick_life: Option<TickLife>,
    pub growable: Option<Growable>,
    pub collision_type: CollisionType,
    pub affected_by_gravity: bool,
    pub updated: bool,
    pub growable_on: bool,
}

#[derive(Clone, Copy)]
pub struct ParticleHealth {
    pub amount: i32,
    pub corrodable: bool,
}

impl ParticleHealth {
    pub fn new(amount: i32, corrodable: bool) -> Self {
        Self { amount, corrodable }
    }
}

impl Default for ParticleHealth {
    fn default() -> Self {
        Self {
            amount: 50,
            corrodable: true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TickLife {
    pub replace_on_death: Option<ParticleTypes>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Velocity {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero_out(&mut self) {
        match self.x.cmp(&0) {
            Ordering::Less => self.x += 1,
            Ordering::Greater => self.x -= 1,
            _ => (),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Density(pub u32);

#[derive(Clone, Copy)]
pub struct Acidity(pub i32);

#[derive(Clone, Copy)]
pub struct Temperature {
    pub current_temperature: i32,
    pub starting_temperature: i32,
    pub coolable: bool,
    pub heatable: bool,
    pub critical_on_cool: bool,
    pub change_on_critical: Option<ParticleTypes>,
    pub explosion_radius: i32,
}

impl Temperature {
    pub fn new(
        starting_temperature: i32,
        coolable: bool,
        heatable: bool,
        critical_on_cool: bool,
        change_on_critical: Option<ParticleTypes>,
        explosion_radius: i32,
    ) -> Self {
        Self {
            starting_temperature,
            current_temperature: starting_temperature,
            coolable,
            heatable,
            critical_on_cool,
            change_on_critical,
            explosion_radius,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TemperatureChanger(pub i32);

#[derive(Clone, Copy)]
pub struct Burnable {
    pub burn_temperature: i32,
    pub burn_ticks: i32,
    pub burn_color: (u8, u8, u8, u8),
    pub cooled_color: (u8, u8, u8, u8),
    pub burning: bool,
}

#[derive(Clone, Copy)]
pub struct Growable {
    pub energy: u32,
    pub spread_chance: f64,
    pub grow_as: ParticleTypes,
    pub up_chance: f64,
    pub can_sprout: bool,
}

impl Growable {
    pub fn new(energy: u32, spread_chance: f64, up_chance: f64, grow_as: ParticleTypes) -> Self {
        Self {
            energy,
            spread_chance,
            grow_as,
            up_chance,
            can_sprout: rand::rng().random_bool(up_chance),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum MovementType {
    Solid,
    #[default]
    Powder,
    Liquid,
    Gas,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CollisionType {
    #[default]
    None,
    Solid,
    Acid,
    Fire,
    Water,
}

impl CollisionType {
    pub fn iter() -> Iter<'static, CollisionType> {
        static COLLISION_TYPES: [CollisionType; 5] = [
            CollisionType::None,
            CollisionType::Solid,
            CollisionType::Acid,
            CollisionType::Fire,
            CollisionType::Water,
        ];
        COLLISION_TYPES.iter()
    }
}
