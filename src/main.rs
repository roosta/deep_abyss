mod debug;
mod player;
mod tilemap;
mod camera;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use debug::DebugPlugin;
use camera::CameraPlugin;

use player::{PlayerBundle, PlayerPlugin};
use tilemap::{TilemapPlugin, WallBundle, ZIndex};

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
            TilemapPlugin,
            PlayerPlugin,
            CameraPlugin
        ))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(ZIndex(0.))
        .add_systems(Startup, setup)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_int_cell::<WallBundle>(1)
        // .init_resource::<LevelWalls>()
        .run();
}
