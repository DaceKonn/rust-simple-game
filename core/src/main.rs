mod plugins;

use bevy::prelude::*;
use plugins::window_setup_plugin::*;
use practice::PracticePlugin;

fn main() {
    let mut app = App::new();
    add_systems(&mut app);
    app.run();
}

fn add_systems(app : &mut App) {

    app
        .add_plugin(WindowSetupPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(PracticePlugin);
}
