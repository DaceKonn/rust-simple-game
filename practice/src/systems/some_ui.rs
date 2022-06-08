use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    
}