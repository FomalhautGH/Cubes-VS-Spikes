use basics::{display_events, WIN_BG_COLOR, WIN_HEIGHT, WIN_WIDTH};
use bevy::{
    core_pipeline::clear_color::ClearColor,
    prelude::{App, Color, CoreStage, Msaa},
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};

mod camera;
mod components;
mod debug;
mod lose_screen;
mod player;
mod points;
mod spikes;
mod walls;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            WIN_BG_COLOR.0,
            WIN_BG_COLOR.1,
            WIN_BG_COLOR.2,
        )))
        .insert_resource(WindowDescriptor {
            title: String::from("Basics"),
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(Msaa::default()) // Rapier
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)) // Rapier
        .add_plugin(camera::CameraPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(walls::WallsPlugin)
        .add_plugin(spikes::SpikesPlugin)
        .add_plugin(points::PointsPlugin)
        .add_plugin(lose_screen::LoseScreenPlugin)
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .run();
}
