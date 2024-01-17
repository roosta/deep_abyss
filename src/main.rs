use bevy::{
    prelude::*,
};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;
use std::collections::{HashMap, HashSet};
use rand::Rng;

// use std::collections::HashSet;

#[derive(Default, Component)]
struct Collider;

const GRID_SIZE: f32 = 16.;
const PLAYER_SIZE: Vec2 = Vec2::new(GRID_SIZE, GRID_SIZE);
const ACCELERATION: f32 = 200.;
const MAX_MOVE_SPEED: f32 = 100.;
const DRAG_FACTOR: f32 = 0.50;

#[derive(Reflect, Default, Debug, Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Default, Debug, Component, Deref, DerefMut)]
struct Direction(Vec2);

#[derive(Default, Component, Reflect)]
struct Player;

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Component, Debug, Deref)]
struct ColliderSize(Vec2);

#[derive(Bundle, Default, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

struct DebugPlugin;

enum CollisionAxis {
    X,
    Y
}

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


/// Represents a wide wall that is 1 tile tall
/// Used to spawn wall collisions
#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
struct Plate {
    left: i32,
    right: i32,
}

/// A simple rectangle type representing a wall of any size
struct WallRect {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

#[derive(Bundle, Default, LdtkIntCell)]
struct ColliderBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    size: ColliderSize,
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
    velocity: Velocity,
    direction: Direction,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

fn update_velocity(
    mut query: Query<(&mut Velocity, &Direction), With<Player>>,
    time: Res<Time>,
) {
    for (mut velocity, direction) in &mut query {
        velocity.x = velocity.x.clamp(-MAX_MOVE_SPEED, MAX_MOVE_SPEED);
        velocity.y = velocity.y.clamp(-MAX_MOVE_SPEED, MAX_MOVE_SPEED);
        velocity.x += direction.x * ACCELERATION * time.delta_seconds();
        velocity.y += direction.y * ACCELERATION * time.delta_seconds();
        velocity.x *= DRAG_FACTOR;
        velocity.y *= DRAG_FACTOR;

    }
}

/// Build up a vector of intersection rects from collider query and player.
fn intersect_rects(
    query: &Query<(&Transform, &ColliderSize), (With<Collider>, Without<Player>)>,
    player_translation: Vec3
) -> Vec<Rect> {

    let mut ret = Vec::new();
    for (transform, size) in query {

        let rect_a = Rect::from_center_size(
            Vec2::new(
                player_translation.x,
                player_translation.y
            ),
            PLAYER_SIZE
        );
        let rect_b = Rect::from_center_size(
            Vec2::new(
                transform.translation.x,
                transform.translation.y
            ),
            size.0
        );

        let intersect_rect = rect_a.intersect(rect_b);
        if !intersect_rect.is_empty() {
            ret.push(intersect_rect);
        }
    }
    ret
}

/// Set player translation for each intesect rect along an axis
fn handle_collisions(
    collider_query: &Query<(&Transform, &ColliderSize), (With<Collider>, Without<Player>)>,
    velocity: &Velocity,
    transform: &mut Transform,
    axis: CollisionAxis,
) {

    for rect in intersect_rects(&collider_query, transform.translation) {
        let size = rect.size();
        match axis {
            CollisionAxis::X => {
                if velocity.x < 0. {
                    transform.translation.x += size.x;
                } else if velocity.x > 0. {
                    transform.translation.x -= size.x;
                }
            },
            CollisionAxis::Y => {
                if velocity.y < 0. {
                    transform.translation.y += size.y;
                } else if velocity.y > 0. {
                    transform.translation.y -= size.y;
                }
            }
        }
    }
}

/// Move player by mutating transform, and check collisions, push player out by the size of the
/// intersect rectangle when collision is detect
fn move_player(
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    collider_query: Query<(&Transform, &ColliderSize), (With<Collider>, Without<Player>)>,
    ) {
    for (mut velocity, mut transform) in &mut query {

        let prev = transform.translation.clone();

        let half_x = velocity.x / 2.;
        let half_y = velocity.y / 2.;

        // Horizontal collisions
        if velocity.x > 0. || velocity.x < 0. {
            transform.translation.x += half_x;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                CollisionAxis::X
            );
            transform.translation.x += half_x;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                CollisionAxis::X
            );
        }

        // Vertical collisions
        if velocity.y > 0. || velocity.y < 0. {
            transform.translation.y += half_y;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                CollisionAxis::Y
            );
            transform.translation.y += half_y;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                CollisionAxis::Y
            );
        }

        // Ensure velocity doesn't trail off in smaller and smaller increments
        if (prev.x - transform.translation.x).abs() < f32::EPSILON {
            velocity.x = 0.;
        }
        if (prev.y - transform.translation.y).abs() < f32::EPSILON {
            velocity.y = 0.;
        }

    }
}

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

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction, With<Player>>
) {
    for mut direction in query.iter_mut() {
        direction.x = 0.;
        direction.y = 0.;
        if keys.pressed(KeyCode::Left)  || keys.pressed(KeyCode::A) {
            direction.x = -1.;
        }
        if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
            direction.x = 1.;
        }
        if keys.pressed(KeyCode::Up)    || keys.pressed(KeyCode::W) {
            direction.y = 1.;
        }
        if keys.pressed(KeyCode::Down)  || keys.pressed(KeyCode::S) {
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
        .add_plugins((
                DefaultPlugins.set(ImagePlugin::default_nearest()),
                DebugPlugin,
                ))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_int_cell::<WallBundle>(1)
        // .init_resource::<LevelWalls>()
        .add_systems(Update,
            (
                handle_input,
                spawn_collisions,
                update_velocity,
                move_player,
            ))
        .run();
}

