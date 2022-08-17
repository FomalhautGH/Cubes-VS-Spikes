use basics::{WALLS_COLOR, WALLS_SIZE, WALL_DISTANCE};
use bevy::{
    prelude::{shape, App, Assets, Color, Commands, Mesh, Name, Plugin, ResMut, Transform, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::components::Wall;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls_system);
    }
}

fn spawn_walls_system(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let walls_y_positions: [f32; 2] = [WALL_DISTANCE, -WALL_DISTANCE];

    for i in 0..2 {
        command
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: Vec2::new(WALLS_SIZE.0, WALLS_SIZE.1),
                        flip: false,
                    }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::rgb(
                    WALLS_COLOR.0,
                    WALLS_COLOR.1,
                    WALLS_COLOR.2,
                ))),
                transform: Transform::from_xyz(0.0, walls_y_positions[i], 2.0),
                ..Default::default()
            })
            .insert(Name::new("Wall"))
            .insert(Wall);
    }
}
