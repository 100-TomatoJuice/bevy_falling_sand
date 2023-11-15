//! The method for generating colliders is based on [Nolla Games' GDC talk on Noita](https://www.youtube.com/watch?v=prXuyMCgbTc)

use bevy::prelude::*;

use self::gen_colliders::generate_sandbox_colliders;

use super::{SANDBOX_X_CHUNKS, SANDBOX_Y_CHUNKS};

pub mod gen_colliders;
mod utils;

pub struct SandboxColliderPlugin;

impl Plugin for SandboxColliderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ColliderStorage::default())
            .add_systems(Update, generate_sandbox_colliders);
    }
}

#[derive(Resource)]
pub struct ColliderStorage {
    pub colliders: Vec<Option<Vec<Entity>>>,
}

impl Default for ColliderStorage {
    fn default() -> Self {
        Self {
            colliders: vec![None; SANDBOX_X_CHUNKS * SANDBOX_Y_CHUNKS],
        }
    }
}
