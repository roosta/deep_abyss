mod debug;
mod player;
mod level;
mod camera;
mod physics;
mod chain;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::PrepareConfig;
use debug::DebugPlugin;
use camera::CameraPlugin;

use player::{PlayerBundle, PlayerPlugin};
use level::{LevelPlugin, TileBundle};
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
            LevelPlugin,
            PlayerPlugin,
            CameraPlugin,
            LdtkPlugin
        ))
        .insert_resource(LevelSelection::index(0))
        // This is a bit of a hack getting xpbd to leave z-index alone, see 1db403b
        .insert_resource(PrepareConfig {
            position_to_transform: false,
            transform_to_position: true,
        })
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..default()
        })
        .add_systems(Startup, setup)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_int_cell::<TileBundle>(1)
        // .init_resource::<LevelWalls>()
        .run();
}
