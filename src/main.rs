use bevy:: prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;

// use std::collections::HashSet;

struct DebugPlugin;

mod player;
mod tilemap;

use player::{
    Player,
    Velocity,
    PlayerBundle,
    PlayerPlugin,
};
use tilemap::{
    WallBundle,
    TilemapPlugin,
    ZIndex,
};

fn _update_print(
    query: Query<(&Velocity, &Transform), With<Player>>,
) {
    for (velocity, transform) in &query {
        println!(
            "Velocity: [{:#?}, {:#?}], [{:#?}, {:#?}]",
            velocity.x,
            velocity.y,
            transform.translation.x,
            transform.translation.y
        )
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins(FilterQueryInspectorPlugin::<With<Player>>::default());
            // app.add_plugins(FilterQueryInspectorPlugin::<With<Collider>>::default());
            app.register_type::<Velocity>();
            // app.add_systems(Update, _update_print);
        }
    }
}




fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 200.;
    camera.transform.translation.y += 150.;
    commands.spawn(camera);
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

