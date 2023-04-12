use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::{Collider, ColliderMassProperties, GravityScale, RigidBody};

use crate::tank::TankBundle;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    println!("window width {} height {}", window.width(), window.height());
    commands.spawn((
        TankBundle::new(
            Vec3::new(window.width() / 2.0, window.height() - 550.0, 0.0),
            asset_server.load("single_sprites/tank_red.png"),
            asset_server.load("single_sprites/tracksSmall.png"),
            asset_server.load("single_sprites/bulletRed2_outline.png"),
            false,
        ),
        //TODO: move collision to TankBundle
        RigidBody::Dynamic,
        GravityScale(0.0),
        ColliderMassProperties::Mass(1000.0),
        Collider::cuboid(25.0, 25.0),
        Enemy,
    ));
}
#[derive(Component)]
struct Enemy;
