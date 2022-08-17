use crate::components::{Player, Wall};
use basics::{PLAYER_COLOR, PLAYER_SCALER, PLAYER_SIZE, PLAYER_SPEED_LIMIT, WALLS_SIZE};
use bevy::{
    prelude::{
        shape, App, Assets, Color, Commands, Input, KeyCode, Local, Mesh, Name, Plugin, Query, Res,
        ResMut, Transform, Vec2, With, Without,
    },
    sprite::{collide_aabb::collide, ColorMaterial, MaterialMesh2dBundle},
    time::Time,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, GravityScale, RigidBody};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system)
            .add_system(movement_player_system)
            .add_system(modify_size_player_system)
            .add_system(increaser_player_speed_system);
    }
}

fn spawn_player_system(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    command
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::splat(PLAYER_SIZE),
                    flip: false,
                }))
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgb(
                PLAYER_COLOR.0,
                PLAYER_COLOR.1,
                PLAYER_COLOR.2,
            ))),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Name::from("Player"))
        .insert(Player { speed: 150.0 })
        .insert(Collider::cuboid(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn movement_player_system(
    mut player_query: Query<(&Player, &mut Transform)>,
    walls_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // Prendo il player e il suo componente Transform
    let (player, mut translation) = player_query.single_mut();
    let transl = &mut translation.translation;

    // Prendo le mura
    let wall_pos: Vec<&Transform> = walls_query.iter().collect();

    // Controllo la presenza di sole due mura.
    if wall_pos.len() != 2 {
        panic!("The number of walls is not 2 somehow.");
    }

    // In base all'input e alla velocità del player stesso lo muovo in alto e in basso.
    if (keyboard.pressed(KeyCode::Up) || keyboard.pressed(KeyCode::W))
        && collide(
            wall_pos[0].translation,
            Vec2::new(WALLS_SIZE.0, WALLS_SIZE.1),
            *transl,
            Vec2::splat(PLAYER_SIZE),
        )
        .is_none()
    {
        transl.y += player.speed * time.delta_seconds();
    }

    if (keyboard.pressed(KeyCode::Down) || keyboard.pressed(KeyCode::S))
        && collide(
            wall_pos[1].translation,
            Vec2::new(WALLS_SIZE.0, WALLS_SIZE.1),
            *transl,
            Vec2::splat(PLAYER_SIZE),
        )
        .is_none()
    {
        transl.y -= player.speed * time.delta_seconds();
    }
}

fn modify_size_player_system(
    mut query: Query<(&mut Transform, &mut Player)>,
    keyboard: Res<Input<KeyCode>>,
    mut mini: Local<bool>,
) {
    if (keyboard.pressed(KeyCode::Left) || keyboard.pressed(KeyCode::A)) && !*mini {
        let (transform_player, player) = &mut query.single_mut();
        let player_scale = &mut transform_player.scale;

        player.speed /= PLAYER_SCALER as f32;
        player_scale.x /= PLAYER_SCALER as f32;
        player_scale.y /= PLAYER_SCALER as f32;

        *mini = true;
    }

    if (keyboard.pressed(KeyCode::Right) || keyboard.pressed(KeyCode::D)) && *mini {
        let (transform_player, player) = &mut query.single_mut();
        let player_scale = &mut transform_player.scale;

        player.speed *= PLAYER_SCALER as f32;
        player_scale.x *= PLAYER_SCALER as f32;
        player_scale.y *= PLAYER_SCALER as f32;

        *mini = false;
    }
}

fn increaser_player_speed_system(mut query: Query<&mut Player>) {
    // Prendo velocità del player.
    let player_speed = query.single_mut().speed;

    // La aumento fino ad un limite.
    if player_speed <= PLAYER_SPEED_LIMIT {
        query.single_mut().speed += 0.01;
    }
}
