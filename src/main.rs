use bevy::prelude::*;

const PLAYER_SPEED: f32 = 400.0;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut)]
struct Direction(Vec2);

#[derive(Component)]
struct Player;

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity, &Direction)>,
    time: Res<Time>,
) {
    for (mut transform, velocity, direction) in &mut query {

        let delta_x = velocity.x * direction.x * time.delta_seconds();
        let delta_y = velocity.y * direction.y * time.delta_seconds();

        transform.translation.x += delta_x;
        transform.translation.y += delta_y;

    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction>
) {
    let mut direction = query.single_mut();
    direction.x = 0.;
    direction.y = 0.;
    if keys.pressed(KeyCode::Left) {
        direction.x = -1.;
    }
    if keys.pressed(KeyCode::Right) {
        direction.x = 1.;
    }
    if keys.pressed(KeyCode::Up) {
        direction.y = 1.;
    }
    if keys.pressed(KeyCode::Down) {
        direction.y = -1.;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("player/placeholder.png"),
        ..default()
    },
    Player,
    Velocity(Vec2::new(PLAYER_SPEED, PLAYER_SPEED)),
    Direction(Vec2::new(0., 0.))
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, apply_velocity).chain())
        .run();
}
