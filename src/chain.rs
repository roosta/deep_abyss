use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_xpbd_2d::{math::*, prelude::*};

// use bevy_ecs_ldtk::prelude::LevelIid;
use crate::player::Player;
use crate::physics::GameLayer;

#[derive(Component)]
struct Anchor;

pub fn spawn(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), Added<Player>>,
    // level_query: Query<(&LevelIid, &Transform), Without<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
        for (entity, transform) in &player_query {
            let particle_radius = 3.0;
            let particle_mesh: Mesh2dHandle = meshes.add(Circle::new(particle_radius as f32)).into();
            let particle_material = materials.add(Color::rgb(0.2, 0.7, 0.9));
            let particle_count = 7;
            let z_index = 5.;
            let collider = Collider::circle(particle_radius);


            // Spawn kinematic particle that acts as anchor, temporarily visible
            let mut previous_particle = commands
                .spawn((
                        Anchor,
                        RigidBody::Kinematic,
                        MaterialMesh2dBundle {
                            mesh: particle_mesh.clone(),
                            material: particle_material.clone(),
                            transform: Transform::from_xyz(
                                transform.translation.x,
                                transform.translation.y + 50.,
                                z_index,
                            ),
                            ..default()
                        },
                        ))
                .id();

            for i in 1..particle_count {
                let current_particle = commands
                    .spawn((
                            RigidBody::Dynamic,
                            MassPropertiesBundle::new_computed(&collider, 1.0),
                            collider.clone(),
                            MaterialMesh2dBundle {
                                mesh: particle_mesh.clone(),
                                material: particle_material.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x,
                                    transform.translation.y - i as f32 * (particle_radius as f32 * 2.0 + 1.0),
                                    z_index,
                                    ),
                                    ..default()
                            },
                            CollisionLayers::new(
                                GameLayer::Chain,
                                [GameLayer::Ground]
                            )

                            ))
                    .id();

                commands.spawn(
                    RevoluteJoint::new(previous_particle, current_particle)
                    .with_local_anchor_2(Vector::Y * (particle_radius * 2.0 + 1.0))
                    .with_linear_velocity_damping(0.1)
                    .with_angular_velocity_damping(1.0)
                    .with_compliance(0.0),
                    );

                previous_particle = current_particle;
            }

            commands.spawn(
                RevoluteJoint::new(previous_particle, entity)
                .with_linear_velocity_damping(0.1)
                .with_angular_velocity_damping(1.0)
                .with_compliance(0.0),
                );
        }
}

