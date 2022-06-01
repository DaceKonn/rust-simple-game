mod systems;

use bevy::prelude::*;
use systems::*;

fn main() {
    let mut app = App::new();
    add_systems(&mut app);
    app.run();
}

fn add_systems(app : &mut App) {
    app
        .add_plugins(DefaultPlugins)
        .add_system(hello_world::hello_world);
}