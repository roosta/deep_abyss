use bevy::{prelude::*, window::WindowResized};
use bevy_ecs_ldtk::prelude::*;

use bevy::render::camera::{OrthographicProjection, ScalingMode, Viewport};

#[derive(Component, Default)]
struct GameViewport;

#[derive(Default, Bundle)]
pub struct CameraBundle {
    marker: GameViewport,
    camera_bundle: Camera2dBundle,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CameraState {
    Manual,
    #[default]
    Auto,
}

pub struct CameraPlugin;

const ASPECT_RATIO: f32 = 16. / 9.;
const MAX_WIDTH: f32 = 384.0;
const MAX_HEIGHT: f32 = 432.0;

use crate::player::Player;
use crate::level::{SURFACE_IID, BOTTOM_IID};

use crate::AppState;

fn setup(mut commands: Commands) {
    let mut camera_game = Camera2dBundle::default();
    camera_game.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: MAX_WIDTH,
        max_height: MAX_HEIGHT,
    };
    commands.spawn(CameraBundle {
        camera_bundle: camera_game,
        ..default()
    });
}

/// Set camera translation based on player pos
fn follow_player(
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

/// Clamp camera to world, ensure that camera stays within the world, and not over extending beyond
/// world bounds
fn clamp_world(
    // level_query: Query<(&Transform, &LevelIid), (Without<GameViewport>, Without<Player>)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<GameViewport>, Without<Player>),
    >,
) {
    let (mut camera_transform, projection) = camera_query.single_mut();
    if let Some(ldtk_project) = ldtk_project_assets.get(ldtk_projects.single()) {
        let surface = ldtk_project
            .get_raw_level_by_iid(&SURFACE_IID.to_string())
            .expect("Unable to get surface level");
        let min = projection.area.max.x;
        let max = surface.px_wid as f32 - projection.area.max.x;
        camera_transform.translation.x = camera_transform.translation.x.clamp(min, max);

        // dbg!(bottom);
        let max = (surface.world_y * -1) as f32 - projection.area.max.y;
        let min = projection.area.max.y;
        camera_transform.translation.y = camera_transform.translation.y.clamp(min, max);
    }

}

/// Debug use only, helps in debugging camera positioning
fn keyboard_control(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<GameViewport>>,
) {
    let left  = keyboard_input.pressed(KeyCode::Numpad4);
    let right = keyboard_input.pressed(KeyCode::Numpad6);
    let up    = keyboard_input.pressed(KeyCode::Numpad8);
    let down  = keyboard_input.pressed(KeyCode::Numpad2);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let speed = 100.0;
    let direction = Vec2::new(horizontal as f32, vertical as f32);
    let mut camera_transform = camera_query.single_mut();
    let delta_time = time.delta_seconds();
    camera_transform.translation.x += direction.x * speed * delta_time;
    camera_transform.translation.y += direction.y * speed * delta_time;

}



/// Set viewport size and position based on camera size (window)
/// Ensures that the viewport takes as much space as possible
fn clamp_viewport(
    mut camera_query: Query<&mut Camera, With<GameViewport>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for _event in resize_reader.read() {
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
}

fn go_to_start(
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<GameViewport>,
    >,
) {

    let (mut camera_transform, projection) = camera_query.single_mut();
    if let Some(ldtk_project) = ldtk_project_assets.get(ldtk_projects.single()) {

        let surface = ldtk_project
            .get_raw_level_by_iid(&SURFACE_IID.to_string())
            .expect("Unable to get surface level");
        let center = surface.px_wid / 2;
        camera_transform.translation.x = center as f32;

        let top = (surface.world_y * -1) as f32 - projection.area.max.y;
        camera_transform.translation.y = top;
    }

}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CameraState>();
        app.add_systems(Startup, setup);
        app.add_systems(Last, clamp_viewport);
        app.add_systems(Update, keyboard_control.run_if(in_state(CameraState::Manual)));
        app.add_systems(OnEnter(AppState::Surface), go_to_start);
        app.add_systems(Update,
            (follow_player, clamp_world)
            .chain()
            .run_if(in_state(CameraState::Auto))
            .run_if(in_state(AppState::Diving))
            );
    }
}
