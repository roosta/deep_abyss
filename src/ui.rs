use bevy::{prelude::*, render::view::visibility::RenderLayers};
use std::time::Duration;

pub struct UiPlugin;


use crate::{AppState, GameAssets};
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

// Component to identify our blinking text
#[derive(Component)]
struct BlinkingText {
    timer: Timer,
    visible: bool,
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

// Sync ui camera with game camera, except for the transform
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

fn setup_start_text(
    mut commands: Commands,
    assets: Res<GameAssets>
) {
    commands
        .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    top: Val::Percent(30.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                RenderLayers::layer(UI_LAYER),
        ))
        .with_children(|parent| {
            parent.spawn((
                    Text::new("Press start"),
                    TextFont {
                        font: assets.font.clone().into(),
                        font_size: 42.0,
                        ..default()
                    },
                    // Because this is a distinct label widget and
                    // not button/list item text, this is necessary
                    // for accessibility to treat the text accordingly.
                    Label,
                    BlinkingText {
                        timer: Timer::new(Duration::from_secs_f32(0.7), TimerMode::Repeating),
                        visible: true,
                    },
                    RenderLayers::layer(UI_LAYER)
            ));
        });
}

// System to handle the blinking effect
fn blink_text(
    time: Res<Time>,
    mut query: Query<(&mut BlinkingText, &mut Visibility)>,
) {
    for (mut blink, mut visibility) in &mut query {
        blink.timer.tick(time.delta());

        if blink.timer.just_finished() {
            blink.visible = !blink.visible;
            *visibility = if blink.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
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
        app.add_systems(OnEnter(AppState::Setup), (setup, setup_start_text));
        app.add_systems(Update, (sync_ui_camera, blink_text));
    }
}
