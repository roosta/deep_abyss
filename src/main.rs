mod debug;
mod player;
mod level;
mod camera;
mod physics;
mod chain;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::PrepareConfig;
use bevy_asset_loader::prelude::*;
use debug::DebugPlugin;
use camera::CameraPlugin;

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
struct LevelAssets {
    #[asset(path = "deep_abyss.ldtk")]
    ldtk_handle: Handle<LdtkProject>,
}

fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut state: ResMut<NextState<AppState>>
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.ldtk_handle.clone(),
        ..Default::default()
    });
    state.set(AppState::Surface);
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
        .add_systems(OnEnter(AppState::Setup), spawn_level)
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Setup)
                .load_collection::<LevelAssets>(),
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
