use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// use bevy_inspector_egui::Inspectable;

use crate::tilemap::{Collider, ColliderSize};

// const GRID_SIZE: f32 = 16.;
const PLAYER_SIZE: Vec2 = Vec2::new(16., 16.);

/// Struct containing values controlling movement, such as gravity and acceleration
#[derive(Default, Component, Reflect, Debug)]
pub struct Physics {
    acceleration: f32,
    move_speed: f32,
    fall_speed: f32,
    drag: f32,
    gravity: f32,
}

// Floating in water values
const FLOATING: Physics = Physics {
    acceleration: 1000.,
    move_speed: 100.,
    fall_speed: 80.,
    drag: 0.80,
    gravity: 35.,
};

// On solid ground
const GROUNDED: Physics = Physics {
    acceleration: 5000.,
    move_speed: 200.,
    fall_speed: 80.,
    drag: 0.50,
    gravity: 35.,
};

#[derive(Default, Component, Reflect)]
pub struct Player;

#[derive(Default, Component, Reflect)]
pub struct OnGround(bool);

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    on_ground: OnGround,
    #[with(init_physics)]
    physics: Physics,
    velocity: Velocity,
    direction: Direction,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

/// Initialize physics values
/// TODO: More will be added here, depending on level design
fn init_physics(_: &EntityInstance) -> Physics {
    FLOATING
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

fn update_velocity(
    mut query: Query<(&mut Velocity, &Direction, &Physics), With<Player>>,
    time: Res<Time>,
) {
    for (mut velocity, direction, physics) in &mut query {
        let Physics {
            acceleration,
            move_speed,
            fall_speed,
            drag,
            gravity,
        } = *physics;
        let slowdown = if direction.y > 0. {
            0.9
        } else if direction.y < 0. {
            1.1
        } else {
            1.
        };
        velocity.y = (velocity.y - gravity * time.delta_seconds()).clamp(-fall_speed, fall_speed);
        velocity.x = (velocity.x + direction.x * acceleration * time.delta_seconds())
            .clamp(-move_speed, move_speed);
        velocity.x *= drag;
        velocity.y *= slowdown;
    }
}

/// Swap out physics component based on OnGround component state
fn check_physics(mut commands: Commands, mut query: Query<(Entity, &OnGround), With<Player>>) {
    for (entity, on_ground) in query.iter_mut() {
        if on_ground.0 {
            commands.entity(entity).insert(GROUNDED);
        } else {
            commands.entity(entity).insert(FLOATING);
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
    on_ground: &mut OnGround,
    axis: CollisionAxis,
) {
    on_ground.0 = false;
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
                let pre = transform.translation.clone();
                if velocity.y < 0. {
                    transform.translation.y += size.y;
                } else if velocity.y > 0. {
                    transform.translation.y -= size.y;
                }
                if pre.y - (PLAYER_SIZE.y / 2.) <= rect.max.y {
                    on_ground.0 = true;
                }
            }
        }
    }
}

/// Move player by mutating transform, and check collisions, push player out by the size of the
/// intersect rectangle when collision is detect
fn move_player(
    mut query: Query<(&mut Velocity, &mut Transform, &mut OnGround), With<Player>>,
    collider_query: Query<(&Transform, &ColliderSize), (With<Collider>, Without<Player>)>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform, mut on_ground) in &mut query {
        let prev = transform.translation.clone();

        let half_x = velocity.x * 0.5 * time.delta_seconds();
        let half_y = velocity.y * 0.5 * time.delta_seconds();

        // Reset ground check
        // on_ground.0 = false;

        // Horizontal collisions
        if velocity.x > 0. || velocity.x < 0. {
            transform.translation.x += half_x;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                &mut on_ground,
                CollisionAxis::X,
            );
            transform.translation.x += half_x;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                &mut on_ground,
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
                &mut on_ground,
                CollisionAxis::Y,
            );
            transform.translation.y += half_y;
            handle_collisions(
                &collider_query,
                &mut velocity,
                &mut transform,
                &mut on_ground,
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
        if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
            direction.y = 1.;
        }
        if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
            direction.y = -1.;
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_input, update_velocity, move_player, check_physics).chain(),
        );
    }
}
