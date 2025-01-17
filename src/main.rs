//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("mario.png")),
        Transform::from_xyz(100., 0., 0.),
        Direction::Up,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {

    let action_record = [ 0].to_vec();
    let mut enums = Vec::<Direction>::new();
    let (mut logo, mut transform) = sprite_position.single_mut();
    for a in action_record {
        match a {
            0 => transform.translation.y += 100. * time.delta_secs(),
            1 => transform.translation.y -= 100. * time.delta_secs(),
            2 => transform.translation.x += 100. * time.delta_secs(),
            3 => transform.translation.x -= 100. * time.delta_secs(),
            _ => (),
        }

        if transform.translation.y > 200. {
            transform.translation.y = 0.;
        } else if transform.translation.y < -200. {
            transform.translation.y = 0.
        }

        if transform.translation.x > 200. {
            transform.translation.x = 0.;
        } else if transform.translation.x < -200. {
            transform.translation.x = 0.;
        }

    }
}
    /* 


    for a in enums {

        let (mut logo, mut transform) = sprite_position.get_single_mut().unwrap();

        *logo = a;
        match *logo {
            Direction::Up => transform.translation.y += 100. * time.delta_secs(),
            Direction::Down => transform.translation.y -= 100. * time.delta_secs(),
            Direction::Right => transform.translation.x += 100. * time.delta_secs(),
            Direction::Left => transform.translation.x -= 100. * time.delta_secs(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }

        if transform.translation.x > 200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. {
            *logo = Direction::Right;
        }

        
    }

    }
    */
    /* 
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 100. * time.delta_secs(),
            Direction::Down => transform.translation.y -= 100. * time.delta_secs(),
            Direction::Right => transform.translation.x += 100. * time.delta_secs(),
            Direction::Left => transform.translation.x -= 100. * time.delta_secs(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }

        if transform.translation.x > 200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. {
            *logo = Direction::Right;
        }
    }
    */
