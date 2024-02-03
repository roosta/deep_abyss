use bevy:: prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};

/// A simple rectangle type representing a wall of any size
struct WallRect {
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

#[derive(Bundle, Default, LdtkIntCell)]
struct ColliderBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    size: ColliderSize,
}

#[derive(Default, Component)]
pub struct Collider;

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Component, Debug, Deref)]
pub struct ColliderSize(pub Vec2);

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

pub struct TilemapPlugin;

// #[derive(Default, Resource)]
// struct LevelWalls {
//     wall_locations: HashSet<GridCoords>,
//     level_width: i32,
//     level_height: i32,
// }


fn spawn_collisions(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    // mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    // collider_query: Query<(Entity, &Transform, &Wall), With<Collider>>,
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

    wall_query.for_each(|(&grid_coords, parent)| {

        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });
    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_iid)| {
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

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
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
                let mut rect_builder: HashMap<Plate, WallRect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<WallRect> = Vec::new();

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
                            .or_insert(WallRect {
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
                        let width = (wall_rect.right as f32 - wall_rect.left as f32 + 1.) * grid_size as f32;
                        let height = (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.) * grid_size as f32;
                        let center_x = (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.;
                        let center_y = (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.;
                        let z_index =  0.;
                        level.spawn(
                            ColliderBundle {
                                sprite_bundle: SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::rgb(red, green, blue),
                                        custom_size: Some(Vec2::new(width, height)),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(center_x, center_y, z_index),
                                    ..default()
                                },
                                size: ColliderSize(Vec2::new(width, height)),
                                collider: Collider
                            },
                        );
                    }
                });
            }
        });
    }
}


impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_collisions);
    }
}
