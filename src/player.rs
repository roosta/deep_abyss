use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// use bevy_inspector_egui::Inspectable;

// use crate::tilemap::ColliderSize;
use bevy_xpbd_2d::{
    prelude::{
        Collider,
        RigidBody,
        Restitution,
        Friction,
        CoefficientCombine,
        LinearVelocity,
        ShapeHits,
        ShapeCaster,
        ColliderDensity,
        GravityScale,
    },
    components::LockedAxes,
    math::{Scalar, Vector}
};

const PLAYER_WIDTH: f32 = 16.0;
const PLAYER_HEIGHT: f32 = 16.0;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// The acceleration used for character movement.
#[derive(Component, Debug)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Debug)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component, Debug)]
pub struct JumpImpulse(Scalar);

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    Jump,
}

#[derive(Default, Component, Reflect)]
pub struct Player;

#[derive(Default, Component, Reflect)]
pub struct OnGround(bool);

#[derive(Bundle)]
pub struct PhysicsBundle {
    rigid_body: RigidBody,
    restitution: Restitution,
    collider: Collider,
    locked_axis: LockedAxes,
    ground_caster: ShapeCaster,
    friction: Friction,
    collider_density: ColliderDensity,
    gravity_scale: GravityScale,

}

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    physics: PhysicsBundle,
    movement: MovementBundle,

    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self {
            acceleration: MovementAcceleration(500.0),
            damping: MovementDampingFactor(0.94),
            jump_impulse: JumpImpulse(400.0)
        }
    }
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        let player_size = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
        let collider = Collider::cuboid(player_size.x, player_size.y);

        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        PhysicsBundle {
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            collider,
            locked_axis: LockedAxes::ROTATION_LOCKED,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Vector::NEG_Y)
                .with_max_time_of_impact(10.0),
            friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            collider_density: ColliderDensity(2.0),
            gravity_scale: GravityScale(1.5),
        }
    }

}


pub struct PlayerPlugin;

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as Scalar;

    if direction != 0.0 {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump);
    }
}

fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut query: Query<(
        &mut LinearVelocity,
        &MovementAcceleration,
        Has<Grounded>
    ), With<Player>>,
) {

    let delta_time = time.delta_seconds();
    for event in movement_event_reader.read() {

        for (mut linear_velocity, acceleration, _is_grounded) in &mut query {
            match event {

                MovementAction::Move(direction) => {
                    linear_velocity.x += *direction * acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    // if is_grounded {
                    //     linear_velocity.y = jump_impulse.0;
                    // }
                }
            }
        }
    }
}

/// Updates the [`Grounded`] status for player
fn update_grounded(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &ShapeHits,
        &mut MovementDampingFactor,
        &mut MovementAcceleration
    ), With<Player>>
) {
    for (entity, hits, mut damping, mut acceleration) in &mut query {

        // let is_grounded = hits.iter().any(|_hit| { true });
        let is_grounded = !hits.is_empty();

        // TODO: Create something cleaner for this, just needed a working commit
        if is_grounded {
            commands.entity(entity).insert(Grounded);
            damping.0 = 0.5;
            acceleration.0 = 5000.0;
        } else {
            commands.entity(entity).remove::<Grounded>();
            damping.0 = 0.94;
            acceleration.0 = 500.0;
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>();
        app.add_systems(Update, (
                keyboard_input,
                update_grounded,
                apply_deferred,
                movement,
                apply_movement_damping,
        ).chain());
    }
}
