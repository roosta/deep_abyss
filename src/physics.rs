use bevy::prelude::*;

use bevy_xpbd_2d::{
    prelude::{
        SubstepCount,
        Gravity,
        PhysicsPlugins
    },
    math::{
        Vector,
        Scalar
    }

};
pub struct PhysicsPlugin;

const GRAVITY: Scalar = 30.0;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.insert_resource(SubstepCount(6));
        app.insert_resource(Gravity(Vector::NEG_Y * GRAVITY));
    }
}
