use bevy::prelude::*;

use avian2d::{
    prelude::{
        SubstepCount,
        Gravity,
        PhysicsPlugins,
        PhysicsLayer
    },
    math::{
        Vector,
        Scalar
    }

};
pub struct PhysicsPlugin;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Player, // Layer 0
    Enemy,  // Layer 1
    Ground, // Layer 2
    Chain   // Layer 3
}

const GRAVITY: Scalar = 30.0;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.insert_resource(SubstepCount(10));
        app.insert_resource(Gravity(Vector::NEG_Y * GRAVITY));
    }
}
