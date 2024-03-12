use bevy::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*, PostProcessCollisions};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SubstepCount(6));
        app.insert_resource(Gravity(Vector::NEG_Y * 1000.0));
    }
}
