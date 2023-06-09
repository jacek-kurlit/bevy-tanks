use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use tanks::bullet::BulletsPlugin;
use tanks::enemy::EnemyPlugin;
use tanks::player::PlayerPlugin;
use tanks::tank::TankPlugin;
use tanks::temporary::TemporaryObjectsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(TankPlugin)
        .add_plugin(BulletsPlugin)
        .add_plugin(TemporaryObjectsPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(WorldInspectorPlugin::default())
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
