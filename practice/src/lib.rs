use bevy::prelude::{Plugin};
use systems::simple_rectangle;

pub mod systems;

pub struct PracticePlugin;

impl Plugin for PracticePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(simple_rectangle::setup);
    }
}