use crate::components::{Player, TextScreen, TextScreenPoints};
use basics::{WIN_HEIGHT, WIN_WIDTH};
use bevy::{
    prelude::{
        shape, App, AssetServer, Assets, Color, Commands, CoreStage, EventReader, Local, Mesh,
        Name, Plugin, Query, Res, ResMut, Transform, Vec2, Without, Visibility,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    text::{Text, Text2dBundle, TextStyle},
};
use bevy_rapier2d::prelude::CollisionEvent;

pub struct LoseScreenPlugin;

impl Plugin for LoseScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, check_lose_system);
    }
}

fn check_lose_system(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lost: Local<bool>,
    mut query_player: Query<&mut Player>,
    mut query_points: Query<(&TextScreenPoints, &mut Visibility), Without<Player>>,
    collision_events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
) {
    if !collision_events.is_empty() && !*lost {
        // Stile font.
        let text_style = TextStyle {
            font: asset_server.load("../assets/chunk_five.ttf"),
            font_size: 60.0,
            color: Color::WHITE,
        };

        // Prendo il player per impedirgli di muoversi.
        let mut player = query_player.single_mut();
        let (points, mut show_points) = query_points.single_mut();

        command
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    format!("Hai perso!\nHai ottenuto {} punti", points.points),
                    text_style,
                ),
                transform: Transform::from_xyz(-195.0, 150.0, 5.0),
                ..Default::default()
            })
            .insert(Name::new("End Screen Text"))
            .insert(TextScreen);

        command
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: Vec2::new(WIN_WIDTH, WIN_HEIGHT),
                        flip: false,
                    }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.97))),
                transform: Transform::from_xyz(0.0, 0.0, 4.0),
                ..Default::default()
            })
            .insert(Name::new("End Screen"));

        player.speed = 0.0;
        show_points.is_visible = false;
        *lost = true;
    }
}
