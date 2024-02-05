use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{
    egui::{
        ScrollArea,
        // CollapsingHeader
        Window,
    },
    EguiContext, EguiPlugin,
};
use bevy_inspector_egui::bevy_inspector::{
    ui_for_world,
    // ui_for_world_entities,
    ui_for_world_entities_filtered,
    // ui_for_resource,
};
// use bevy_inspector_egui::prelude::*;
// use std::any::TypeId;
pub struct DebugPlugin;

use crate::player::{Direction, Player, Velocity};

use crate::tilemap::{Collider, ZIndex};

/// Change z_index of all colliders on z_index change
fn apply_z_index(z_index: Res<ZIndex>, mut query: Query<&mut Transform, With<Collider>>) {
    if z_index.is_changed() {
        for mut transform in &mut query {
            transform.translation.z = z_index.0
        }
    }
}

fn _update_print(query: Query<(&Velocity, &Transform), With<Player>>) {
    for (velocity, transform) in &query {
        println!(
            "Velocity: [{:#?}, {:#?}], [{:#?}, {:#?}]",
            velocity.x, velocity.y, transform.translation.x, transform.translation.y
        )
    }
}

/// API: https://github.com/emilk/egui
fn inspector_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    let mut fps: f64 = 0.;
    let diagnostics = world.get_resource::<DiagnosticsStore>();

    // FPS
    match diagnostics {
        Some(diag) => {
            match diag
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
            {
                Some(value) => {
                    fps = value;
                }
                _ => (),
            }
        }
        None => {
            warn!("Unable to get DiagnosticsStore, FPS counter will not work")
        }
    }
    Window::new("Deep Abyss: Debug Inspector").show(egui_context.get_mut(), |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            ui_for_world(world, ui);
            ui_for_world_entities_filtered::<With<Player>>(world, ui, false);

            if let Some(mut z_index) = world.get_resource_mut::<ZIndex>() {
                if ui.button("Toggle collision boxes").clicked() {
                    if z_index.0 == 0. {
                        z_index.0 = 50.;
                    } else if z_index.0 == 50. {
                        z_index.0 = 0.
                    }
                }
            }

            ui.label(format!("FPS: {}", fps.round()));
            // ui.heading("State");
            // ui.label("z index");
            // ui_for_resource::<ZIndex>(world, ui);

            // CollapsingHeader::new("State").show(ui, |ui| {
            //     ui.label("z index");
            //     ui_for_resource::<ZIndex>(world, ui);
            // });
            // ui.heading("Entities");
            // bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}

// TODO: Only start on dev
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // if cfg!(debug_assertions) {
        app.add_plugins(EguiPlugin);
        app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin); // adds default options and `InspectorEguiImpl`s
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Update, (inspector_ui, apply_z_index));
        app.register_type::<Velocity>();
        app.register_type::<ZIndex>();
        app.register_type::<Direction>();
    }
    // }
}
