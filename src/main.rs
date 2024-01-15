use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 32.;
const GRID_SIZE: i32 = 16;

#[derive(Default, Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Default, Component, Deref, DerefMut)]
struct Direction(Vec2);

#[derive(Default, Component)]
struct Player;


#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

// #[derive(Default, Resource)]
// struct LevelWalls {
//     wall_locations: HashSet<GridCoords>,
//     level_width: i32,
//     level_height: i32,
// }

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[with(init_velocity)]
    velocity: Velocity,
    // #[with(init_direction)]
    direction: Direction,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

fn init_velocity(_: &EntityInstance) -> Velocity {
    Velocity(Vec2::new(PLAYER_SPEED, PLAYER_SPEED))
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity, &Direction), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, velocity, direction) in &mut query {

        let pos_x = transform.translation.x  + velocity.x * direction.x * time.delta_seconds();
        let pos_y = transform.translation.y + velocity.y * direction.y * time.delta_seconds();

        transform.translation.x = pos_x;
        transform.translation.y = pos_y;
    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction, With<Player>>
) {
    for mut direction in query.iter_mut() {
        direction.x = 0.;
        direction.y = 0.;
        if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
            direction.x = -1.;
        }
        if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
            direction.x = 1.;
        }
        if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
            direction.y = 1.;
        }
        if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
            direction.y = -1.;
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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .register_ldtk_entity::<PlayerBundle>("Player")
        // .register_ldtk_int_cell::<WallBundle>(1)
        // .init_resource::<LevelWalls>()
        .add_systems(Update, (handle_input, apply_velocity).chain())
        .run();
}

