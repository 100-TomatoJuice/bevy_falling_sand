use bevy::prelude::*;

use self::gen_colliders::generate_sandbox_colliders;

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
            colliders: vec![None; 17 * 30],
        }
    }
}
