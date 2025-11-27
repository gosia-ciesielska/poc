mod game_plugin;
mod player_control_system;
mod item_system;

use crate::game_plugin::GamePlugin;
use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(GamePlugin)
        .insert_resource(Gravity(Vec2::NEG_Y*0.0))
        .run();
}
