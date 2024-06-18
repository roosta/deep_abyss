use bevy::{
    prelude::*,
    window::WindowResized,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy::render::camera::ScalingMode;

#[derive(Component, Default)]
struct OverlayViewport;

#[derive(Component, Default)]
struct Element {
    width: f32,
    height: f32,
}

const PADDING: f32 = 6.0;

pub struct OverlayPlugin;
use crate::camera::{
    MAX_HEIGHT,
    ASPECT_RATIO,
    ViewportChange
};

use bevy::render::view::visibility::RenderLayers;

#[derive(Default, Bundle)]
pub struct CameraBundle {
    marker: OverlayViewport,
    camera_bundle: Camera2dBundle,
    render_layers: RenderLayers,
}

#[derive(Default, Bundle)]
pub struct ElementBundle {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    element: Element,
    render_layers: RenderLayers,
}

fn setup_portrait(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let srcery_green = Color::hex("#519F50").unwrap();
    let material = materials.add(srcery_green);
    let width = 16.0;
    let height = 16.0;
    let mesh = Mesh2dHandle(meshes.add(Rectangle::new(width, height)));
    commands.spawn(ElementBundle {
        mesh_bundle: MaterialMesh2dBundle {
            mesh,
            material,
            ..default()
        },
        element: Element { width, height },
        render_layers: RenderLayers::layer(1),


    });
}

fn setup(
    mut commands: Commands,
) {
    let mut camera_overlay = Camera2dBundle {
        camera: Camera {
            order: 1,
            clear_color: ClearColorConfig::None, // do not use a clear color
            ..default()
        },
        ..default()
    };
    camera_overlay.projection.scaling_mode = ScalingMode::FixedVertical(MAX_HEIGHT);
    commands.spawn(CameraBundle {
        camera_bundle: camera_overlay,
        render_layers: RenderLayers::layer(1),
        ..default()
    });
}

fn calc_position(
    mut reader: EventReader<ViewportChange>,
    mut query: Query<(&mut Transform, &Element)>,
    camera_query: Query<&OrthographicProjection, With<OverlayViewport>>,
) {
    for _event in reader.read() {
        let projection = camera_query.single();
        let left = projection.area.min.y / ASPECT_RATIO;
        for (mut transform, element) in &mut query {
            transform.translation.x = left - (element.width / 2.0) - PADDING;
        }
    }
}
impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_portrait));
        app.add_systems(Update, calc_position);
    }
}
