use crate::components::{Point, TextScreenPoints};
use bevy::{
    prelude::{App, AssetServer, Color, Commands, Name, Plugin, Query, Res, ResMut, Transform},
    text::{Text, Text2dBundle, TextStyle},
    time::{Time, Timer},
};
use std::time::Duration;

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_points_system)
            .add_startup_system(spawn_point_counter_system)
            .add_system(points_system);
    }
}

fn points_system(
    mut query: Query<(&mut Text, &mut TextScreenPoints)>,
    mut timer_points: ResMut<Point>,
    time: Res<Time>,
) {
    timer_points.timer.tick(time.delta());

    if timer_points.timer.just_finished() {
        let (text_points, points) = &mut query.single_mut();

        text_points.sections[0].value = format!("{}", points.points);
        points.points += 1;
    }
}

fn spawn_point_counter_system(mut command: Commands, asset_server: Res<AssetServer>) {
    // Stile font.
    let text_style = TextStyle {
        font: asset_server.load("../assets/chunk_five.ttf"),
        font_size: 60.0,
        color: Color::WHITE,
    };

    command
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0", text_style),
            transform: Transform::from_xyz(0.0, 310.0, 5.0),
            ..Default::default()
        })
        .insert(Name::new("Point text"))
        .insert(TextScreenPoints { points: 0 });
}

fn setup_points_system(mut command: Commands) {
    command.insert_resource(Point {
        timer: Timer::new(Duration::from_secs(1), true),
    });
}
