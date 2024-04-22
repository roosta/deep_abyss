mod debug;
mod player;
mod tilemap;
mod camera;
mod physics;
mod chain;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::PrepareConfig;
use debug::DebugPlugin;
use camera::CameraPlugin;

use player::{PlayerBundle, PlayerPlugin};
use tilemap::{TilemapPlugin, TileBundle, ZIndex};
use physics::PhysicsPlugin;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("deep_abyss.ldtk"),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            DebugPlugin,
            PhysicsPlugin,
            TilemapPlugin,
            PlayerPlugin,
            CameraPlugin,
            LdtkPlugin
        ))
        .insert_resource(LevelSelection::index(0))
        .insert_resource(ZIndex(0.))
        .insert_resource(PrepareConfig {
            position_to_transform: false,
            transform_to_position: true,
        })
        .add_systems(Startup, setup)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_int_cell::<TileBundle>(1)
        // .init_resource::<LevelWalls>()
        .run();
}
