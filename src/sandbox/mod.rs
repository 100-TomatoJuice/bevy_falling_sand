use bevy::{
    prelude::*,
    render::{render_resource::*, texture::ImageSampler},
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
    spawn_sandbox(&mut commands, &mut images, 240, 136);
}

pub fn spawn_sandbox(commands: &mut Commands, images: &mut Assets<Image>, width: u32, height: u32) {
    let image_handle = {
        let mut image = Image::new_fill(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[0, 0, 0, 0],
            TextureFormat::Rgba8UnormSrgb,
        );
        image.sampler_descriptor = ImageSampler::nearest();
        images.add(image)
    };
    commands
        .spawn(Sandbox::new(30, 17, 8, 8))
        .insert(SpriteBundle {
            texture: image_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(8.0, 8.0, 1.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, PI, PI),
            },
            ..Default::default()
        });
}