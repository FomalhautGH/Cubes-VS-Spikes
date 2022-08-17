use bevy::prelude::EventReader;
use bevy_rapier2d::prelude::CollisionEvent;

type ColorRGB = (f32, f32, f32);

pub const WIN_WIDTH: f32 = 1080.0;
pub const WIN_HEIGHT: f32 = 720.0;
pub const WIN_BG_COLOR: ColorRGB = (0.0, 0.0, 0.0);

pub const PLAYER_SIZE: f32 = 70.0;
pub const PLAYER_SPEED_LIMIT: f32 = 300.0;
pub const PLAYER_COLOR: ColorRGB = (255.0, 0.0, 0.0);
pub const PLAYER_SCALER: u8 = 2;

pub const WALLS_COLOR: ColorRGB = (0.0, 0.0, 255.0);
pub const WALLS_SIZE: (f32, f32) = (WIN_WIDTH, WIN_HEIGHT / 2.0);
pub const WALL_DISTANCE: f32 = 360.0;

pub const SPIKE_COLOR: ColorRGB = (0.0, 255.0, 0.0);
pub const SPIKE_SPAWN_POINT: f32 = 700.0;

pub fn display_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }
}

pub fn menu_system() {}
