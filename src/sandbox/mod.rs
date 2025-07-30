use bevy::{
    prelude::*,
    image::ImageSampler,
    render::{render_asset::RenderAssetUsages, render_resource::*},
    time::common_conditions::on_timer,
};
use std::f32::consts::PI;
use std::time::Duration;

use self::{
    collider::SandboxColliderPlugin, particle_placer::ParticlePlacerPlugin,
    render::render_particles, sandbox::Sandbox, simulation::update_particles,
};

mod chunk;
pub mod collider;
mod effects;
pub mod particle;
mod particle_placer;
pub mod particle_types;
mod render;
pub mod sandbox;
mod simulation;

const SANDBOX_CHUNK_WIDTH: usize = 8;
const SANDBOX_CHUNK_HEIGHT: usize = 8;
const SANDBOX_X_CHUNKS: usize = 30;
const SANDBOX_Y_CHUNKS: usize = 17;

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParticlePlacerPlugin)
            .add_plugins(SandboxColliderPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (update_particles, render_particles)
                    .chain()
                    .distributive_run_if(on_timer(Duration::from_secs_f32(1.0 / 24.0))),
            );
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    spawn_sandbox(
        &mut commands,
        &mut images,
        SANDBOX_X_CHUNKS,
        SANDBOX_Y_CHUNKS,
    );
}

pub fn spawn_sandbox(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    x_chunks: usize,
    y_chunks: usize,
) {
    let image_handle = {
        let mut image = Image::new_fill(
            Extent3d {
                width: (x_chunks * SANDBOX_CHUNK_WIDTH) as u32,
                height: (y_chunks * SANDBOX_CHUNK_HEIGHT) as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[0, 0, 0, 0],
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.sampler = ImageSampler::nearest();
        images.add(image)
    };

    commands
        .spawn(Sandbox::new(
            x_chunks,
            y_chunks,
            SANDBOX_CHUNK_WIDTH,
            SANDBOX_CHUNK_HEIGHT,
        ))
        .insert((Sprite {
            image: image_handle,
            ..Default::default()
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(8.0, 8.0, 1.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, PI, PI),
            },
        ));
}
