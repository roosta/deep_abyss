use bevy::{prelude::*, render::view::visibility::RenderLayers};

pub struct UiPlugin;

use crate::camera::{
    GameCamera,
    UI_LAYER
};

#[derive(Component, Default)]
struct UiCamera;

#[derive(Bundle)]
pub struct CameraBundle {
    marker: UiCamera,
    camera: Camera,
    camera_2d: Camera2d,
    render_layers: RenderLayers,
}

fn setup_border(mut commands: Commands) {
    // Create a border using four rectangles
    commands
        .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                RenderLayers::layer(UI_LAYER),
        ))
        .with_children(|parent| {
            // Top border
            parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Px(2.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
                    RenderLayers::layer(UI_LAYER),
            ));

            // Bottom border
            parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Px(2.0),
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
                    RenderLayers::layer(UI_LAYER),
            ));

            // Left border
            parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(2.0),
                        height: Val::Percent(100.0),
                        left: Val::Px(0.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
                    RenderLayers::layer(UI_LAYER),
            ));

            // Right border
            parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(2.0),
                        height: Val::Percent(100.0),
                        right: Val::Px(0.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
                    RenderLayers::layer(UI_LAYER),
            ));
        });
}
// Modified sync system that only syncs viewport and projection
fn sync_ui_camera(
    game_camera: Query<(&Camera, &OrthographicProjection), (With<GameCamera>, Changed<OrthographicProjection>)>,
    mut ui_camera: Query<(&mut Camera, &mut OrthographicProjection), (With<UiCamera>, Without<GameCamera>)>,
) {
    if let (Ok((game_cam, game_proj)), Ok((mut ui_cam, mut ui_proj))) =
        (game_camera.get_single(), ui_camera.get_single_mut()) {
            // Sync viewport
            ui_cam.viewport = game_cam.viewport.clone();

        // Sync projection
        ui_proj.viewport_origin = game_proj.viewport_origin;
        ui_proj.scaling_mode = game_proj.scaling_mode.clone();
        ui_proj.scale = game_proj.scale;
    }
}

fn setup(mut commands: Commands) {
    let camera = Camera {
        clear_color: ClearColorConfig::None,
        order: UI_LAYER as isize ,
        ..default()
    };
    let camera_2d = Camera2d { ..default() };
    commands.spawn(CameraBundle {
        marker: UiCamera,
        camera,
        camera_2d,
        render_layers: RenderLayers::layer(UI_LAYER),
    });
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_border));
        app.add_systems(Update, sync_ui_camera);
    }
}
