use bevy::{prelude::Component, time::Timer};
use bevy_inspector_egui::Inspectable;

// Player components
#[derive(Component, Inspectable)]
pub struct Player {
    pub speed: f32,
}
// =================================================================

//Spikes components
#[derive(Component, Inspectable)]
pub struct Spike;

//Spikes resources
pub struct SpikeSpawnConfig {
    pub speed: f32,
    pub x_distance: f32,
}
// =================================================================

//Walls components
#[derive(Component, Inspectable)]
pub struct Wall;
// =================================================================

// Text components
#[derive(Component, Inspectable)]
pub struct TextScreen;

#[derive(Component, Inspectable)]
pub struct TextScreenPoints {
    pub points: u32,
}
// =================================================================

// Point resource
pub struct Point {
    pub timer: Timer,
}
// =================================================================
