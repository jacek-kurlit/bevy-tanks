use bevy::{prelude::*, window::PrimaryWindow};
use tanks::bullet::BulletsPlugin;
use tanks::player::PlayerPlugin;
use tanks::temporary::TemporaryObjectsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletsPlugin)
        .add_plugin(TemporaryObjectsPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
        ..default()
    });
    println!("window width {} height {}", window.width(), window.height());
}
