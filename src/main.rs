use bevy::prelude::*;

mod hello_world;

fn main() {
    let mut app = App::new();
    add_systems(&mut app);
    app.run();
}

fn add_systems(app : &mut App) {
    app.add_system(hello_world::hello_world);
}