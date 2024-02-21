use bevy::prelude::*;

use bevy::render::camera::{
    ScalingMode,
    Viewport,
};

#[derive(Component, Default)]
struct GameViewport;



#[derive(Default, Bundle)]
pub struct CameraBundle {
    marker: GameViewport,
    camera_bundle: Camera2dBundle,
}
pub struct CameraPlugin;

const ASPECT_RATIO: f32 = 16. / 9.;
const MAX_WIDTH: f32 = 384.0;
const LEVEL_WIDTH: f32 = 768.0;
const MAX_HEIGHT: f32 = 432.0;

use crate::player::Player;

fn setup(mut commands: Commands) {
    let mut camera_game = Camera2dBundle::default();
    camera_game.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: MAX_WIDTH,
        max_height: MAX_HEIGHT
    };
    commands.spawn(CameraBundle {
        camera_bundle: camera_game,
        ..default()
    });
}

fn follow_player(
    mut camera_query: Query<&mut Transform, (With<GameViewport>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {

    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_translation.x;
        camera_transform.translation.y = player_translation.y;

    }
}

fn clamp_viewport(
    mut camera_query: Query<&mut Camera, (With<GameViewport>, Without<Player>)>,
    ) {
    let mut camera = camera_query.single_mut();
    if let Some(size) = camera.physical_target_size() {
        let size_x = size.x as f32;
        let size_y = size.y as f32;
        let max_width = size_y / ASPECT_RATIO;
        let center_x = (size_x / 2.) - (max_width / 2.);
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(center_x as u32, 0),
            physical_size: UVec2::new(max_width as u32, size.y),
            ..default()

        })
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (clamp_viewport, follow_player));
    }
}
