use bevy::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

pub struct WindowSetupPlugin {
    height: f32
}

impl Default for WindowSetupPlugin {
    fn default() -> Self {
        Self { height: 900.0 }
    }
}

impl Plugin for WindowSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(CLEAR))
            .insert_resource(WindowDescriptor {
                width: self.height * RESOLUTION,
                height: self.height,
                title: "Rust Simple Game".to_string(),
                resizable: false,
                ..Default::default()
            });
    }
}