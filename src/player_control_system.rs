use avian2d::prelude::{LinearVelocity};
use bevy::math::Vec2;
use bevy::prelude::{ButtonInput, Component, KeyCode, Query, Res, Time, With};

#[derive(Component)]
pub struct PlayerControl;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut LinearVelocity, With<PlayerControl>>
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let move_delta = direction.normalize_or_zero() * 5000.0 * time.delta_secs();
    for mut velocity in query.iter_mut() {
        velocity.0 += move_delta;
    }
}