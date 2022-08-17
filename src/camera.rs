use bevy::prelude::{App, Camera2dBundle, Commands, Name, Plugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut command: Commands) {
    command
        .spawn_bundle(Camera2dBundle {
            ..Default::default()
        })
        .insert(Name::new("Camera_2D"));
}
