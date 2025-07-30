use bevy::prelude::*;

use super::sandbox::Sandbox;

const BACKGROUND_COLOR: (u8, u8, u8, u8) = (0, 0, 0, 0);

// © 2021 Bas van Schoonhoven
// Based on https://github.com/grunnt/falling-rust/blob/master/src/render.rs
pub fn render_particles(
    mut images: ResMut<Assets<Image>>,
    mut sandbox: Query<(&mut Sandbox, &Sprite)>,
) {
    let (mut sandbox, image_handle) = sandbox
        .single_mut()
        .expect("Sandbox should be created by this point");

    if let Some(image) = images.get_mut(&image_handle.image) {
        for y in 0..sandbox.height() {
            for x in 0..sandbox.width() {
                if !sandbox.get_chunk(x, y).is_strong_ticked() {
                    continue;
                }

                let particle = sandbox.get_mut(x, y);
                let color = match particle {
                    Some(particle) => particle.color,
                    None => BACKGROUND_COLOR,
                };

                let bytes_per_pixel = 4;
                let index = (x + y * sandbox.width()) * bytes_per_pixel;

                let data = image.data.as_mut().unwrap();
                data[index] = color.0;
                data[index + 1] = color.1;
                data[index + 2] = color.2;
                data[index + 3] = color.3;
            }
        }
    }
}
