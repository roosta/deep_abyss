use bevy:: prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub struct DebugPlugin;

use crate::player::{
    Player,
    Velocity,
};


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

