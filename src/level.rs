use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use rand::Rng;
use std::collections::{HashMap, HashSet};
use avian2d::prelude::{
    RigidBody,
    Collider,
    CollisionLayers,
};

use crate::{AppState, GameAssets};
use crate::player::Player;
use crate::physics::GameLayer;

pub const SURFACE_IID: &str = "be8059a0-fec0-11ee-a7c8-3d38ef9d3a0f";
pub const BOTTOM_IID: &str = "f450ce20-fec0-11ee-a3f3-2d25d2c76d98";

/// A simple rectangle type representing a wall of any size
struct TileRect {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

/// Represents a wide wall that is 1 tile tall
/// Used to spawn wall collisions
#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
struct Plate {
    left: i32,
    right: i32,
}

#[derive(Bundle)]
struct ColliderBundle {
    sprite: Sprite,
    transform: Transform,
    size: ColliderSize,
    rigid_body: RigidBody,
    collider: Collider,
    wall: Wall,
    collision_layers: CollisionLayers
}

#[derive(Component, Default)]
pub struct Wall;

#[derive(Default, Component)]
struct Tile;

#[derive(Default, Component, Debug, Deref)]
pub struct ColliderSize(pub Vec2);

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TileBundle {
    tile: Tile,
}

pub struct LevelPlugin;

// #[derive(Default, Resource)]
// struct LevelWalls {
//     wall_locations: HashSet<GridCoords>,
//     level_width: i32,
//     level_height: i32,
// }

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
///
/// Source: https://github.com/Trouv/bevy_ecs_ldtk/blob/201d908ae3e4f3deeb40de228f234c414c6b3141/examples/platformer/systems.rs#L62-L229
///
/// Modified roosta<mail@roosta.sh>
/// Modified to use avian physics engine instead of rapier
fn spawn_collisions(
    mut commands: Commands,
    tile_query: Query<(&GridCoords, &Parent), Added<Tile>>,
    parent_query: Query<&Parent, Without<Tile>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    // mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    // collider_query: Query<(Entity, &Transform, &Tile), With<Collider>>,
) {
    let mut rng = rand::thread_rng();

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    tile_query.iter().for_each(|(&grid_coords, parent)| {
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });
    if !tile_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right
                    // edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, TileRect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<TileRect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(TileRect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        let red: f32 = rng.gen_range(0.0..1.0);
                        let green: f32 = rng.gen_range(0.0..1.0);
                        let blue: f32 = rng.gen_range(0.0..1.0);
                        let width = (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                            * grid_size as f32;
                        let height = (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                            * grid_size as f32;
                        let center_x =
                            (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.;
                        let center_y =
                            (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.;
                        level.spawn(ColliderBundle {
                            sprite: Sprite {
                                color: Color::srgb(red, green, blue),
                                custom_size: Some(Vec2::new(width, height)),
                                ..default()
                            },
                            transform: Transform::from_xyz(center_x, center_y, 0.0),
                            size: ColliderSize(Vec2::new(width, height)),
                            collider: Collider::rectangle(width, height),
                            rigid_body: RigidBody::Static,
                            wall: Wall::default(),
                            collision_layers: CollisionLayers::new(
                                GameLayer::Ground,
                                [GameLayer::Player, GameLayer::Chain]
                            )
                        });
                    }
                });
            }
        });
    }
}

/// Modify level selection based on player position
/// TODO: Unload previous levels
fn follow_player(
    players: Query<&GlobalTransform, With<Player>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if let Ok(player_transform) = players.get_single() {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("ldtk project should be loaded before player is spawned");

        for (level_iid, level_transform) in levels.iter() {
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("level should exist in only project");

            let level_bounds = Rect {
                min: Vec2::new(
                    level_transform.translation().x,
                    level_transform.translation().y,
                ),
                max: Vec2::new(
                    level_transform.translation().x + level.px_wid as f32,
                    level_transform.translation().y + level.px_hei as f32,
                ),
            };

            let target = player_transform.translation().truncate();
            if level_bounds.contains(target) {
                *level_selection = LevelSelection::Iid(level_iid.clone());
            }
        }
    }
}


fn spawn_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut state: ResMut<NextState<AppState>>
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.level.clone().into(),
        ..Default::default()
    });
    state.set(AppState::Surface);
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_collisions, follow_player));
        app.add_systems(OnEnter(AppState::Setup), spawn_level);
    }
}
