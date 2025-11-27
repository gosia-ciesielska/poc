use crate::item_system::{Item, PickupMessage, UseMessage};
use avian2d::prelude::LinearVelocity;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{
    ButtonInput, ChildOf, Component, Entity, KeyCode, MessageWriter, Query, Res, Time, Transform,
    With, Without,
};

#[derive(Component)]
pub struct PlayerControl;

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &mut Transform), With<PlayerControl>>,
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

    direction = direction.normalize_or_zero();

    let move_delta = direction * 5000.0 * time.delta_secs();

    for (mut velocity, mut transform)in query.iter_mut() {
        velocity.0 += move_delta;
        if direction != Vec2::ZERO {
            transform.rotation = Quat::from_rotation_z(direction.to_angle());
            if transform.translation.x > 300.0 { transform.translation.x = -290.0; }
            if transform.translation.x < -300.0 { transform.translation.x = 290.0; }
            if transform.translation.y > 300.0 { transform.translation.y = -290.0; }
            if transform.translation.y < -300.0 { transform.translation.y = 290.0; }
        }
    }
}

pub fn player_item_pickup_system(
    player_query: Query<(Entity, &Transform), With<PlayerControl>>,
    items_query: Query<(Entity, &Transform), (With<Item>, Without<ChildOf>)>,
    mut message_writer: MessageWriter<PickupMessage>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single() {
        crate::item_system::scan_and_pickup_nearby(
            player_entity,
            player_transform,
            &items_query,
            &mut message_writer,
            36.0,
        );
    }
}

pub fn player_item_use_system(
    items_query: Query<(Entity, &ChildOf), With<Item>>,
    players_query: Query<(), With<PlayerControl>>,
    mut message_writer: MessageWriter<UseMessage>,
) {
    for (item_entity, child_of) in items_query.iter() {
        if players_query.contains(child_of.0) {
            message_writer.write(UseMessage {
                item: item_entity,
            });
        }
    }
}
