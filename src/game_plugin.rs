use crate::item_system::{Item, PickupMessage, UseMessage, handle_item_pickup, shoot_gun, Gun};
use crate::player_control_system::{
    PlayerControl, player_item_pickup_system, player_item_use_system, player_movement_system,
};
use avian2d::prelude::*;
use bevy::color::palettes::css::{BLUE, CORAL, DARK_GRAY};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement_system)
            .add_systems(
                Update,
                player_item_pickup_system.run_if(input_just_pressed(KeyCode::KeyE)),
            )
            .add_message::<PickupMessage>()
            .add_systems(Update, handle_item_pickup)
            .add_message::<UseMessage>()
            .add_systems(
                Update,
                player_item_use_system.run_if(input_just_pressed(KeyCode::Space)),
            )
            .add_systems(Update, shoot_gun);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(500.0, 10.0),
        Mesh2d(meshes.add(Rectangle::new(500.0, 10.0))),
        MeshMaterial2d(materials.add(Color::from(DARK_GRAY))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Restitution::new(0.4),
        Collider::rectangle(20.0, 20.0),
        Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
        MeshMaterial2d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(0.0, 100.0, 0.0),
        LinearDamping(16.0),
        AngularDamping(8.0),
        MaxLinearSpeed(256.0),
        PlayerControl,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(5.0, 10.0))),
        Collider::rectangle(5.0, 10.0),
        MeshMaterial2d(materials.add(Color::from(CORAL))),
        Transform::from_xyz(240.0, 150.0, 0.0),
        Item,
        Gun
    ));

    commands.spawn((
        Text::new(
            "Movement: WSAD\nPick up item: E\nUse item: Spacebar",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}
