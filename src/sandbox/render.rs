use bevy::prelude::*;

use super::sandbox::Sandbox;

const BACKGROUND_COLOR: (u8, u8, u8, u8) = (0, 0, 0, 0);

pub fn render_particles(
    mut images: ResMut<Assets<Image>>,
    mut sandbox: Query<(&mut Sandbox, &Handle<Image>)>,
) {
    let (mut sandbox, image_handle) = sandbox
        .get_single_mut()
        .expect("Sandbox should be created by this point");

    let image = images.get_mut(image_handle).unwrap();
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

            image.data[index] = color.0;
            image.data[index + 1] = color.1;
            image.data[index + 2] = color.2;
            image.data[index + 3] = color.3;
        }
    }
}
