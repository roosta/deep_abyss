use bevy::prelude::*;

use bevy::render::camera::ScalingMode;

#[derive(Component)]
struct GameViewport;

pub struct CameraPlugin;

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

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
