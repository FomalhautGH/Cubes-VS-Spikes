use crate::components::{Spike, SpikeSpawnConfig};
use basics::{SPIKE_COLOR, SPIKE_SPAWN_POINT, WALL_DISTANCE};
use bevy::{
    prelude::{
        shape, App, Assets, Color, Commands, Entity, Local, Mesh, Name, Plugin, Quat, Query, Res,
        ResMut, Transform, Vec2, Vec3, With,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    time::Time,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, GravityScale, RigidBody};
use rand::Rng;
use std::f32::consts::PI;

pub struct SpikesPlugin;

impl Plugin for SpikesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_spikes_spawning_system)
            .add_system(movement_spikes_system)
            .add_system(spawn_spikes_system);
    }
}

fn spawn_spikes_system(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut config: ResMut<SpikeSpawnConfig>,
    mut prev_radius: Local<f32>,
    query: Query<&Transform, With<Spike>>,
) {
    let mut spawn_spike = |np: i8, radians: f32| {
        let radius = rand::thread_rng().gen_range(10.0..(200.0 - *prev_radius / 2.0));

        command
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::RegularPolygon { radius, sides: 3 }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::rgb(
                    SPIKE_COLOR.0,
                    SPIKE_COLOR.1,
                    SPIKE_COLOR.2,
                ))),
                transform: Transform {
                    translation: Vec3::new(
                        SPIKE_SPAWN_POINT,
                        WALL_DISTANCE / 2.0 * (np as f32),
                        0.0,
                    ),
                    rotation: Quat::from_rotation_z(radians),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Spike"))
            .insert(Spike)
            .insert(Collider::triangle(
                Vec2::new(-radius / 1.7, 0.0),
                Vec2::new(radius / 1.7, 0.0),
                Vec2::new(0.0, radius),
            ))
            .insert(RigidBody::Fixed)
            .insert(GravityScale(0.0))
            .insert(ActiveEvents::COLLISION_EVENTS);

        radius
    };

    let pos_spikes: Vec<&Transform> = query.iter().collect();
    match pos_spikes.last() {
        Some(pos_last_spike) => {
            if config.x_distance <= (SPIKE_SPAWN_POINT - pos_last_spike.translation.x) {
                *prev_radius = if rand::random::<bool>() {
                    spawn_spike(1, PI)
                } else {
                    spawn_spike(-1, 0.0)
                };

                config.x_distance = rand::thread_rng().gen_range(175.0..300.0);
            }
        }
        None => *prev_radius = spawn_spike(-1, 0.0),
    }
}

fn movement_spikes_system(
    mut command: Commands,
    mut query: Query<(Entity, &mut Transform), With<Spike>>,
    config: Res<SpikeSpawnConfig>,
    time: Res<Time>,
) {
    for (e_spike, mut spike_pos) in query.iter_mut() {
        spike_pos.translation.x -= config.speed * time.delta_seconds();

        if spike_pos.translation.x <= -SPIKE_SPAWN_POINT {
            command.entity(e_spike).despawn();
        }
    }
}

// Configurazione spawning di spikes.
fn setup_spikes_spawning_system(mut command: Commands) {
    command.insert_resource(SpikeSpawnConfig {
        x_distance: 150.0,
        speed: 400.0,
    });
}
