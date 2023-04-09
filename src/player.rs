use crate::{
    bullet::spawn_bullet,
    tank::{Tank, TankAmmunitation, TankBundle},
    tracks::Tracks,
};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(handle_player_input);
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
            Vec3::new(window.width() / 2.0, 50.0, 0.0),
            asset_server.load("single_sprites/tank_blue.png"),
            asset_server.load("single_sprites/tracksSmall.png"),
            asset_server.load("single_sprites/bulletBlue2_outline.png"),
        ),
        Player,
    ));
}

#[derive(Component)]
struct Player;
fn handle_player_input(
    mut player_query: Query<(&mut Transform, &Tank, &TankAmmunitation, &mut Tracks), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (mut player_transform, tank, tank_ammunition, mut tracks) = player_query.single_mut();
    let mut movement_direction: Option<Vec3> = None;
    let mut rotation_direction: Option<f32> = None;
    if keyboard.pressed(KeyCode::W) {
        movement_direction = Some(player_transform.up());
    }
    if keyboard.pressed(KeyCode::S) {
        movement_direction = Some(player_transform.down());
    }
    if keyboard.pressed(KeyCode::D) {
        rotation_direction = Some(-1.0);
    }
    if keyboard.pressed(KeyCode::A) {
        rotation_direction = Some(1.0);
    }
    if let Some(direction) = movement_direction {
        player_transform.translation += direction * tank.move_speed * time.delta_seconds();
        tracks.update_tracks(&player_transform, &mut commands);
    }
    if let Some(rotation) = rotation_direction {
        player_transform.rotate_z(rotation * tank.rotation_speed * time.delta_seconds());
    }
    if keyboard.just_pressed(KeyCode::Space) {
        spawn_bullet(*player_transform, tank_ammunition, &mut commands);
    }
}
