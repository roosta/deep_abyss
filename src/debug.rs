use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{
    egui::{
        ScrollArea,
        Align2,
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
use crate::level::Wall;


const HIDDEN_Z: f32 = 0.0;
const VISIBLE_Z: f32 = 50.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Event)]
enum WallColliderVisibility {
    Visible,
    #[default]
    Hidden
}

#[derive(Default, Resource)]
struct DebugState {
    camera_control: CameraState,
    wall_visibility: WallColliderVisibility
}

/// Change z_index of all colliders on z_index change
fn apply_z_index(
    mut query: Query<&mut Transform, With<Wall>>,
    mut reader: EventReader<WallColliderVisibility>,
) {

    for event in reader.read() {
        match event {
            WallColliderVisibility::Visible => {
                for mut transform in &mut query {
                    transform.translation.z = VISIBLE_Z;
                }
            },
            WallColliderVisibility::Hidden => {
                for mut transform in &mut query {
                    transform.translation.z = HIDDEN_Z;
                }
            }
        }
    }
}

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
        Window::new("Deep Abyss: Debug Inspector")
            .default_open(false)
            .anchor(Align2::LEFT_BOTTOM, [0.0, 0.0])
            .show(context.get_mut(), |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui_for_world(world, ui);
                    ui_for_world_entities_filtered::<With<Player>>(world, ui, false);

                    ui.horizontal(|ui| {
                        ui.label("Frames per second");
                        ui.label(fps.round().to_string());
                    });

                    ui.horizontal(|ui| {
                        ui.label("Collision boxes");
                        if ui.add(SelectableLabel::new(local.wall_visibility == WallColliderVisibility::Hidden, "Hidden")).clicked() {
                            local.wall_visibility = WallColliderVisibility::Hidden;
                            world.send_event(WallColliderVisibility::Hidden);
                        }
                        if ui.add(SelectableLabel::new(local.wall_visibility == WallColliderVisibility::Visible, "Visible")).clicked() {
                            local.wall_visibility = WallColliderVisibility::Visible;
                            world.send_event(WallColliderVisibility::Visible);
                        }
                    });

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
                });
            });
    }

}

// TODO: Only start on dev
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // if cfg!(debug_assertions) {
        app.add_plugins(EguiPlugin);
        app.add_event::<WallColliderVisibility>();
        app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Update, (inspector_ui, apply_z_index));
        }
    // }
}
