use bevy::prelude::Entity;

use super::particle::Particle;

const MAX_TICKED_BEFORE_SLEEP: u8 = 2;

#[derive(Clone)]
pub struct SandboxChunk {
    width: usize,
    height: usize,
    pub local_position: (usize, usize),
    particles: Vec<Option<Particle>>,
    pub colliders: Vec<Entity>,
    strong_ticked: u8,
    weak_ticked: u8,
}

impl SandboxChunk {
    pub fn new(width: usize, height: usize, index: usize) -> Self {
        Self {
            width,
            height,
            particles: vec![None; width * height],
            local_position: {
                let local_x = index % 30;
                let local_y = index / 30;

                (local_x, local_y)
            },
            colliders: vec![],
            strong_ticked: MAX_TICKED_BEFORE_SLEEP,
            weak_ticked: MAX_TICKED_BEFORE_SLEEP,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Particle> {
        let index = self.to_index(x, y);
        self.particles[index].as_ref()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Particle> {
        let index = self.to_index(x, y);
        self.particles[index].as_mut()
    }

    pub fn set(&mut self, x: usize, y: usize, particle: Option<Particle>) {
        let index = self.to_index(x, y);
        self.particles[index] = particle;

        self.strong_tick();
    }

    pub fn reset_ticked(&mut self) {
        self.strong_ticked = self.strong_ticked.saturating_sub(1);
        self.weak_ticked = self.weak_ticked.saturating_sub(1);
    }

    pub fn strong_tick(&mut self) {
        self.strong_ticked = MAX_TICKED_BEFORE_SLEEP;
    }

    pub fn weak_tick(&mut self) {
        self.weak_ticked = MAX_TICKED_BEFORE_SLEEP;
    }

    pub fn is_strong_ticked(&self) -> bool {
        self.strong_ticked > 0
    }

    pub fn is_weak_ticked(&self) -> bool {
        self.weak_ticked > 0
    }

    pub fn mark_updated(&mut self, x: usize, y: usize) {
        let index = self.to_index(x, y);
        let particle = self.particles[index].as_mut();
        if let Some(particle) = particle {
            particle.updated = true;
        }
    }

    pub fn reset_updated(&mut self) {
        for particle in self.particles.iter_mut().filter_map(|x| x.as_mut()) {
            particle.updated = false;
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }
}
