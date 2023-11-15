use bevy::prelude::Component;

use super::{chunk::SandboxChunk, particle::Particle};

#[derive(Component)]
pub struct Sandbox {
    x_chunks: usize,
    y_chunks: usize,
    chunk_width: usize,
    chunk_height: usize,
    total_width: usize,
    total_height: usize,
    chunks: Vec<SandboxChunk>,
}

impl Sandbox {
    pub fn new(x_chunks: usize, y_chunks: usize, chunk_width: usize, chunk_height: usize) -> Self {
        Self {
            x_chunks,
            y_chunks,
            chunk_width,
            chunk_height,
            total_width: x_chunks * chunk_width,
            total_height: y_chunks * chunk_height,
            chunks: {
                let mut chunks = Vec::with_capacity(x_chunks * y_chunks);
                for i in 0..chunks.capacity() {
                    chunks.push(SandboxChunk::new(chunk_width, chunk_height, i, x_chunks));
                }
                chunks
            },
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Particle> {
        let index = self.to_index(x, y);
        self.chunks.get(index)?.get(x % self.chunk_width, y % self.chunk_height)
    }

    pub fn checked_get(&self, x: usize, y: usize) -> Option<&Particle> {
        if self.out_of_bounds_usize(x, y) {
            None
        } else {
            self.get(x, y)
        }
    }

    pub fn checked_get_i32(&self, x: i32, y: i32) -> Option<&Particle> {
        if self.out_of_bounds_i32(x, y) {
            None
        } else {
            self.get(x as usize, y as usize)
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Particle> {
        let index = self.to_index(x, y);
        self.chunks.get_mut(index)?.get_mut(x % self.chunk_width, y % self.chunk_height)
    }

    pub fn checked_get_mut(&mut self, x: usize, y: usize) -> Option<&mut Particle> {
        if self.out_of_bounds_usize(x, y) {
            None
        } else {
            self.get_mut(x, y)
        }
    }

    pub fn checked_get_mut_i32(&mut self, x: i32, y: i32) -> Option<&mut Particle> {
        if self.out_of_bounds_i32(x, y) {
            None
        } else {
            self.get_mut(x as usize, y as usize)
        }
    }

    pub fn set(&mut self, x: usize, y: usize, particle: Option<Particle>) {
        let index = self.to_index(x, y);
        if index >= self.chunks.len(){
            return;
        }
        
        self.chunks[index].set(x % self.chunk_width, y % self.chunk_height, particle);

        self.strong_tick_neighbors(x, y);
    }

    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let index1 = self.to_index(x1, y1);
        let index2 = self.to_index(x2, y2);

        let particle1 = self.chunks[index1]
            .get(x1 % self.chunk_width, y1 % self.chunk_height)
            .copied();
        let particle2 = self.chunks[index2]
            .get(x2 % self.chunk_width, y2 % self.chunk_height)
            .copied();

        self.chunks[index1].set(x1 % self.chunk_width, y1 % self.chunk_height, particle2);
        self.chunks[index2].set(x2 % self.chunk_width, y2 % self.chunk_height, particle1);

        self.strong_tick_neighbors(x1, y1);
        self.strong_tick_neighbors(x2, y2);
    }

    pub fn get_chunk(&self, x: usize, y: usize) -> &SandboxChunk {
        let index = self.to_index(x, y);
        &self.chunks[index]
    }

    pub fn get_chunk_mut(&mut self, x: usize, y: usize) -> &mut SandboxChunk {
        let index = self.to_index(x, y);
        &mut self.chunks[index]
    }

    pub fn get_all_chunks(&self) -> &[SandboxChunk] {
        &self.chunks
    }

    fn strong_tick_neighbors(&mut self, x: usize, y: usize) {
        let search_directions = [
            (x.overflowing_sub(1).0, y),
            (x + 1, y),
            (x, y.overflowing_sub(1).0),
            (x, y + 1),
            (x.overflowing_sub(1).0, y.overflowing_sub(1).0),
            (x + 1, y + 1),
            (x + 1, y.overflowing_sub(1).0),
            (x.overflowing_sub(1).0, y + 1),
        ];

        for (neighbor_x, neighbor_y) in search_directions.into_iter() {
            if self.out_of_bounds_usize(neighbor_x, neighbor_y) {
                continue;
            }

            self.get_chunk_mut(neighbor_x, neighbor_y).strong_tick();
        }
    }

    pub fn mark_updated(&mut self, x: usize, y: usize) {
        let index = self.to_index(x, y);
        self.chunks[index].mark_updated(x % self.chunk_width, y % self.chunk_height);
    }

    pub fn reset_updated(&mut self) {
        for chunk in self.chunks.iter_mut() {
            chunk.reset_updated();
        }
    }

    pub fn reset_ticked_chunks(&mut self) {
        for chunk in self.chunks.iter_mut() {
            chunk.reset_ticked();
        }
    }

    pub fn eight_surrounded(&self, x: usize, y: usize) -> bool {
        let search_directions = [
            (x.overflowing_sub(1).0, y),
            (x + 1, y),
            (x, y.overflowing_sub(1).0),
            (x, y + 1),
            (x.overflowing_sub(1).0, y.overflowing_sub(1).0),
            (x + 1, y + 1),
            (x + 1, y.overflowing_sub(1).0),
            (x.overflowing_sub(1).0, y + 1),
        ];

        for (neighbor_x, neighbor_y) in search_directions {
            if self.checked_get(neighbor_x, neighbor_y).is_none()
                && !self.out_of_bounds_usize(neighbor_x, neighbor_y)
            {
                return false;
            }
        }

        true
    }

    pub fn width(&self) -> usize {
        self.total_width
    }

    pub fn height(&self) -> usize {
        self.total_height
    }

    pub fn out_of_bounds_i32(&self, x: i32, y: i32) -> bool {
        x < 0 || x >= self.total_width as i32 || y < 0 || y >= self.total_height as i32
    }

    pub fn out_of_bounds_usize(&self, x: usize, y: usize) -> bool {
        x >= self.total_width || y >= self.total_height
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        ((y / self.chunk_height) * self.x_chunks) + x / self.chunk_width
    }
}
