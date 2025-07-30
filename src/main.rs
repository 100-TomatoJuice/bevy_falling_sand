use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_rapier2d::prelude::*;

mod sandbox;
mod vector;
use sandbox::SandboxPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Falling Sand".into(),
                        resolution: (1280., 720.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(SandboxPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
       Camera2d::default(),
       Msaa::Off,
       Projection::from(OrthographicProjection {
           scaling_mode: ScalingMode::Fixed {
               width: 1920.0,
               height: 1080.0,
           },
           near: -1000.0,
           ..OrthographicProjection::default_2d()
       }),
   ));
}
