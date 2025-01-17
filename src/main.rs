//! Renders a 2D scene containing a single, moving sprite.

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    resolution: WindowResolution::new(1920.0, 1080.0)
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("/home/temp/Github_Projects/bevy_sprite/assets/pacman.png")),
        Transform::from_xyz(100., 0., 0.),
        Direction::Up,
    ));
}

pub const ACTIONS: [i32; 15] = [0, 1, 0, 2, 3, 3, 3, 3, 4, 0, 1, 2, 1, 5, 10];
/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (logo, mut transform) in &mut sprite_position {
        //  match *logo {
        //      Direction::Up => transform.translation.y += 150. * time.delta_secs(),
        //      Direction::Down => transform.translation.y -= 150. * time.delta_secs(),
        //  }

        let i = (time.elapsed_secs() as f64 / time.delta_secs_f64()) as usize % 15;
        let my_move = ACTIONS[i];

        match my_move {
            0 => transform.translation.y += 1.,
            1 => transform.translation.y -= 1.,
            2 => transform.translation.x += 1.,
            3 => transform.translation.x -= 1.,
            _ => (),
        }

        if transform.translation.y > 540. {
            transform.translation.y = 0.;
        } else if transform.translation.y < -540. {
            transform.translation.y = 0.;
        }

        if transform.translation.x > 910. {
            transform.translation.x = 0.;
        } else if transform.translation.x < -910. {
            transform.translation.x = 0.;
        }
    }
}
