mod debug;
mod player;
mod level;
mod camera;
mod physics;
mod chain;
mod overlay;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use avian2d::prelude::PrepareConfig;
use bevy_asset_loader::prelude::*;
use debug::DebugPlugin;
use camera::CameraPlugin;
use overlay::OverlayPlugin;

use level::{LevelPlugin, TileBundle};
use player::{
    // PlayerBundle,
    PlayerPlugin
};
use physics::PhysicsPlugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Loading,
    Setup,
    Surface,
    Diving,
    Paused,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "fonts/PixelifySans-Regular.ttf")]
    font: Handle<Font>,

    #[asset(path = "deep_abyss.ldtk")]
    level: Handle<LdtkProject>,
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
            OverlayPlugin,
            LdtkPlugin
        ))
        .insert_resource(LevelSelection::index(0))
        // This is a bit of a hack getting xpbd to leave z-index alone, see 1db403b
        .insert_resource(PrepareConfig {
            position_to_transform: false,
            transform_to_position: true,
        })
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Setup)
                .load_collection::<GameAssets>(),
        )
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..default()
        })
        // .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_int_cell::<TileBundle>(1)
        // .init_resource::<LevelWalls>()
        .init_state::<AppState>()
        .run();
}
