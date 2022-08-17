use crate::components::{Player, Spike};
use bevy::prelude::{App, Plugin};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .add_plugin(RapierDebugRenderPlugin::default())
                .register_inspectable::<Player>()
                .register_inspectable::<Spike>();
        }
    }
}
