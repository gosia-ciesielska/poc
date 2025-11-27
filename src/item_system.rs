
use avian2d::prelude::{
    AngularDamping, Collider, LinearDamping, LinearVelocity, MaxLinearSpeed,
    Restitution, RigidBody,
};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::color::palettes::css::RED;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{ChildOf, Circle, ColorMaterial, Commands, Component, Entity, GlobalTransform, MeshMaterial2d, Message, MessageReader, MessageWriter, Query, ResMut, Transform, Vec3Swizzles, With, Without};

#[derive(Message)]
pub struct PickupMessage {
    pub owner: Entity,
    pub item: Entity,
}

#[derive(Message)]
pub struct UseMessage {
    pub item: Entity,
}

#[derive(Component)]
pub struct Item;

pub fn handle_item_pickup(
    mut commands: Commands,
    mut message_reader: MessageReader<PickupMessage>,
) {
    for message in message_reader.read() {
        commands.entity(message.owner).add_child(message.item);
        commands.entity(message.item).insert(Transform::IDENTITY);
    }
}

pub fn scan_and_pickup_nearby(
    owner: Entity,
    owner_transform: &Transform,
    items_query: &Query<(Entity, &Transform), (With<Item>, Without<ChildOf>)>,
    message_writer: &mut MessageWriter<PickupMessage>,
    range: f32,
) {
    let mut closest_item = None;
    let mut min_dist = range;

    for (item_entity, item_transform) in items_query.iter() {
        let dist = owner_transform
            .translation
            .distance(item_transform.translation);
        if dist < min_dist {
            min_dist = dist;
            closest_item = Some(item_entity);
        }
    }

    if let Some(item) = closest_item {
        message_writer.write(PickupMessage { owner, item });
    }
}

#[derive(Component)]
pub struct Gun; 

pub fn shoot_gun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut message_reader: MessageReader<UseMessage>,
    global_transforms: Query<&GlobalTransform>,
) {
    for message in message_reader.read() {
        if let Ok(global_transform) = global_transforms.get(message.item) {
            let direction = global_transform.right().xy();
            commands.spawn((
                RigidBody::Dynamic,
                Restitution::new(0.8),
                Collider::circle(10.0),
                Mesh2d(meshes.add(Circle::new(10.0))),
                MeshMaterial2d(materials.add(Color::from(RED))),
                Transform::from_translation((global_transform.translation().xy() + direction*10.0).extend(0.0)),
                LinearDamping(0.01),
                AngularDamping(0.01),
                MaxLinearSpeed(1024.0),
                LinearVelocity( direction * 256.0),
            ));
        }
    }
}
