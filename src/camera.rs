use bevy::prelude::*;

use bevy::render::camera::ScalingMode;

#[derive(Component)]
struct GameViewport;

pub struct CameraPlugin;

use crate::player::Player;

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        camera: Camera {
            order: 2,
            ..default()
        },
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 768.0,
        max_height: 432.0,
    };
    commands.spawn((camera, GameViewport));
}

fn clamp_camera(
    mut camera_query: Query<&mut Transform, (With<GameViewport>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_translation.x;
        camera_transform.translation.y = player_translation.y;
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, clamp_camera);
    }
}
