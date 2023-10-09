use bevy::prelude::*;

use super::{
    particle_types::{get_particle, ParticleTypes},
    sandbox::Sandbox,
};

pub struct ParticlePlacerPlugin;

impl Plugin for ParticlePlacerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedParticle {
            particle_type: ParticleTypes::Sand,
        })
        .add_systems(Update, place_particles);
    }
}

#[derive(Resource)]
pub struct SelectedParticle {
    particle_type: ParticleTypes,
}

pub fn place_particles(
    mut sandbox_query: Query<&mut Sandbox>,
    query_window: Query<&Window>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut selected: ResMut<SelectedParticle>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window: &Window = query_window.get_single().unwrap();
    let mut sandbox = sandbox_query.single_mut();

    if let Some(particle_type) = set_particle_type(keyboard_input) {
        selected.particle_type = particle_type;
    }

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let x = ((world_position.x / 8.0) + (sandbox.width() / 2) as f32) as usize;
        let y = ((world_position.y / 8.0) + (sandbox.height() / 2) as f32) as usize;

        if sandbox.out_of_bounds_usize(x, y) {
            return;
        }

        for x_offset in -5..5{
            for y_offset in -5..5{
                let x = x.saturating_add_signed(x_offset);
                let y = y.saturating_add_signed(y_offset);

                if mouse_button_input.pressed(MouseButton::Left) && sandbox.get(x, y).is_none() {
                    sandbox.set(x, y, Some(get_particle(selected.particle_type)));
                } else if mouse_button_input.pressed(MouseButton::Right) && sandbox.get(x, y).is_some() {
                    sandbox.set(x, y, None);
                }
            }
        }
    }
}

fn set_particle_type(keyboard_input: Res<Input<KeyCode>>) -> Option<ParticleTypes> {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        println!("Chose Sand");
        return Some(ParticleTypes::Sand);
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        println!("Chose Water");
        return Some(ParticleTypes::Water);
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        println!("Chose Stone");
        return Some(ParticleTypes::Stone);
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        println!("Chose Acid");
        return Some(ParticleTypes::Acid);
    } else if keyboard_input.just_pressed(KeyCode::Key5) {
        println!("Chose Wood");
        return Some(ParticleTypes::Wood);
    } else if keyboard_input.just_pressed(KeyCode::Key6) {
        println!("Chose Spark");
        return Some(ParticleTypes::Spark);
    } else if keyboard_input.just_pressed(KeyCode::Key7) {
        println!("Chose Lava");
        return Some(ParticleTypes::Lava);
    } else if keyboard_input.just_pressed(KeyCode::Key8) {
        println!("Chose Oil");
        return Some(ParticleTypes::Oil);
    } else if keyboard_input.just_pressed(KeyCode::Key9) {
        println!("Chose Gunpowder");
        return Some(ParticleTypes::Gunpowder);
    } else if keyboard_input.just_pressed(KeyCode::Key0) {
        println!("Chose TNT");
        return Some(ParticleTypes::Tnt);
    } else if keyboard_input.just_pressed(KeyCode::Minus) {
        println!("Chose Dirt");
        return Some(ParticleTypes::Dirt);
    } else if keyboard_input.just_pressed(KeyCode::Equals) {
        println!("Chose Grass");
        return Some(ParticleTypes::Grass);
    }

    None
}
