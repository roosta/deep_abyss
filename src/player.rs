use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// use bevy_inspector_egui::Inspectable;

use crate::tilemap::{Collider, ColliderSize};

const GRID_SIZE: f32 = 16.;
const PLAYER_SIZE: Vec2 = Vec2::new(GRID_SIZE, GRID_SIZE);
const MOVE_ACCELERATION: f32 = 100.;
const MAX_MOVE_SPEED: f32 = 200.;
const DRAG_FACTOR: f32 = 0.90;
const ACCELERATION_FACTOR: f32 = 1.1;
const GRAVITY_ACCELERATON: f32 = 35.0;
const MAX_FALL_SPEED: f32 = 80.0;

#[derive(Default, Component, Reflect)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    velocity: Velocity,
    direction: Direction,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

enum CollisionAxis {
    X,
    Y,
}

#[derive(Reflect, Default, Debug, Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Reflect, Default, Debug, Component, Deref, DerefMut)]
pub struct Direction(Vec2);

pub struct PlayerPlugin;

fn update_velocity(mut query: Query<(&mut Velocity, &Direction), With<Player>>, time: Res<Time>) {
    for (mut velocity, direction) in &mut query {
        velocity.x = velocity.x.clamp(-MAX_MOVE_SPEED, MAX_MOVE_SPEED);
        velocity.y = velocity.y.clamp(-MAX_FALL_SPEED, MAX_FALL_SPEED);
        velocity.x += direction.x * MOVE_ACCELERATION * ACCELERATION_FACTOR * time.delta_seconds();
        velocity.y -= GRAVITY_ACCELERATON * time.delta_seconds();
        if direction.x == 0. {
            velocity.x *= DRAG_FACTOR;
        }
    }
}

/// Build up a vector of intersection rects from collider query and player.
fn intersect_rects(
    query: &Query<(&Transform, &ColliderSize), (With<Collider>, Without<Player>)>,
    player_translation: Vec3,
) -> Vec<Rect> {
    let mut ret = Vec::new();
    for (transform, size) in query {
        let rect_a = Rect::from_center_size(
            Vec2::new(player_translation.x, player_translation.y),
            PLAYER_SIZE,
        );
        let rect_b = Rect::from_center_size(
            Vec2::new(transform.translation.x, transform.translation.y),
            size.0,
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
            }
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
    time: Res<Time>

) {
    for (mut velocity, mut transform) in &mut query {
        let prev = transform.translation.clone();

        let half_x = velocity.x * 0.5 * time.delta_seconds();
        let half_y = velocity.y * 0.5 * time.delta_seconds();

        // Horizontal collisions
        if velocity.x > 0. || velocity.x < 0. {
            transform.translation.x += half_x;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                CollisionAxis::X,
            );
            transform.translation.x += half_x;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                CollisionAxis::X,
            );
        }

        // Vertical collisions
        if velocity.y > 0. || velocity.y < 0. {
            transform.translation.y += half_y;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                CollisionAxis::Y,
            );
            transform.translation.y += half_y;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                CollisionAxis::Y,
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

fn handle_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Direction, With<Player>>) {
    for mut direction in query.iter_mut() {
        direction.x = 0.;
        direction.y = 0.;
        if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
            direction.x = -1.;
        }
        if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
            direction.x = 1.;
        }
        // if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
        //     direction.y = 1.;
        // }
        // if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
        //     direction.y = -1.;
        // }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_input, update_velocity, move_player));
    }
}
