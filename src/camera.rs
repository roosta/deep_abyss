use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy::render::camrra::{OrthographicProjection, ScalingMode, Viewport};

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
const MAX_HEIGHT: f32 = 432.0;

use crate::player::Player;

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

fn clamp_level(
    level_query: Query<(&Transform, &LevelIid), (Without<GameViewport>, Without<Player>)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<GameViewport>, Without<Player>),
    >,
) {
    let (mut camera_transform, projection) = camera_query.single_mut();
    for (_level_transform, level_iid) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");
        if level_selection.is_match(&LevelIndices::default(), level) {
            let min = projection.area.max.x;
            let max = level.px_wid as f32 - projection.area.max.x;
            camera_transform.translation.x = camera_transform.translation.x.clamp(min, max);

            let max = level.px_hei as f32 - projection.area.max.y;
            let min = projection.area.max.y;
            camera_transform.translation.y = camera_transform.translation.y.clamp(min, max);
        }
    }
}

/// Set viewport size and position based on camera size (window)
/// Ensures that the viewport takes as much space as possible
/// TODO: Run only on window resize
fn clamp_viewport(mut camera_query: Query<&mut Camera, With<GameViewport>>) {
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
        app.add_systems(Update, clamp_viewport);
        app.add_systems(Update, (follow_player, clamp_level).chain());
    }
}
