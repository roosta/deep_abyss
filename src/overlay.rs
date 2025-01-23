use bevy::{
    prelude::*,
    sprite::{MeshMaterial2d},
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

#[derive(Bundle)]
pub struct CameraBundle {
    marker: OverlayViewport,
    projection: OrthographicProjection,
    camera: Camera,
    render_layers: RenderLayers,
}

#[derive(Default, Bundle)]
pub struct ElementBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    element: Element,
    render_layers: RenderLayers,
}

fn setup_portrait(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // let srcery_green = Srgba::hex("#519F50").unwrap();
    let material = materials.add(Color::srgb(0.2, 0.7, 0.9));
    let width = 16.0;
    let height = 16.0;
    let mesh = meshes.add(Rectangle::new(width, height));
    commands.spawn(ElementBundle {
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material),
        element: Element { width, height },
        render_layers: RenderLayers::layer(1),


    });
}

/// Setup overlay camera, using fixed vertical scaling, so that we can draw on the sides of the
/// game viewport.
fn setup(
    mut commands: Commands,
) {
    let camera_overlay = Camera {
        order: 1,
        clear_color: ClearColorConfig::None,
        ..default()
    };
    let mut projection = OrthographicProjection {
        ..OrthographicProjection::default_2d()
    };
    projection.scaling_mode = ScalingMode::FixedVertical { viewport_height: MAX_HEIGHT };
    commands.spawn(CameraBundle {
        marker: OverlayViewport,
        camera: camera_overlay,
        projection,
        render_layers: RenderLayers::layer(1),
    });
}


/// Calculate positions for overlay elements. Runs on viewport change, and sets the x position of
/// each element
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
