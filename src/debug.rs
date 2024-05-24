use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{
    egui::{
        ScrollArea,
        SelectableLabel,
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

use crate::player::Player;

use crate::camera::CameraState;
use crate::level::{ZIndex, Wall};


#[derive(Default, Resource)]
struct DebugState {
    camera_control: CameraState
}

/// Change z_index of all colliders on z_index change
fn apply_z_index(z_index: Res<ZIndex>, mut query: Query<&mut Transform, With<Wall>>) {
    if z_index.is_changed() {
        for mut transform in &mut query {
            transform.translation.z = z_index.0
        }
    }
}

// fn _update_print(query: Query<(&Velocity, &Transform), With<Player>>) {
//     for (velocity, transform) in &query {
//         println!(
//             "Velocity: [{:#?}, {:#?}], [{:#?}, {:#?}]",
//             velocity.x, velocity.y, transform.translation.x, transform.translation.y
//         )
//     }
// }

/// API: https://github.com/emilk/egui
fn inspector_ui(
    world: &mut World,
    mut local: Local<DebugState>
) {
    let mut query = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>();
    if let Ok(egui_context) = query.get_single(world) {
        let mut context = egui_context.clone();
        let mut fps: f64 = 0.;
        let diagnostics = world.get_resource::<DiagnosticsStore>();

        // FPS
        match diagnostics {
            Some(diag) => {
                match diag
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
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
        Window::new("Deep Abyss: Debug Inspector").show(context.get_mut(), |ui| {
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

                let mut next = world.get_resource_mut::<NextState<CameraState>>().unwrap();
                ui.horizontal(|ui| {
                    ui.label("Camera control");
                    if ui.add(SelectableLabel::new(local.camera_control == CameraState::Auto, "Auto")).clicked() {
                        next.set(CameraState::Auto);
                        local.camera_control = CameraState::Auto;
                    }
                    if ui.add(SelectableLabel::new(local.camera_control == CameraState::Manual, "Manual")).clicked() {
                        next.set(CameraState::Manual);
                        local.camera_control = CameraState::Manual;
                    }
                    });
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

}

// TODO: Only start on dev
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // if cfg!(debug_assertions) {
        app.add_plugins(EguiPlugin);
        app.insert_resource(ZIndex(0.));
        app.insert_resource(DebugState {
            camera_control: CameraState::Auto
        });
        app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Update, (inspector_ui, apply_z_index));
        app.register_type::<ZIndex>();
        app.register_type::<Direction>();
    }
    // }
}
