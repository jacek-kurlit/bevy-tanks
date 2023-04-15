use crate::{
    bullet::TankAmmunitation,
    tank::{TankBundle, TankMovementIntention},
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
            true,
        ),
        Name::new("Player"),
        Player,
    ));
}

#[derive(Component)]
struct Player;
fn handle_player_input(
    mut player_query: Query<
        (
            &mut Transform,
            &mut TankMovementIntention,
            &mut TankAmmunitation,
        ),
        With<Player>,
    >,
    keyboard: Res<Input<KeyCode>>,
) {
    let (player_transform, mut tank_movement_intention, mut tank_ammunition) =
        player_query.single_mut();
    let mut movement_velocity: Vec2 = Vec2::default();
    let mut rotation_velocity = 0.0;
    if keyboard.pressed(KeyCode::W) {
        let up = player_transform.up();
        movement_velocity = Vec2::new(up.x, up.y);
    }
    if keyboard.pressed(KeyCode::S) {
        let down = player_transform.down();
        movement_velocity = Vec2::new(down.x, down.y);
    }
    if keyboard.pressed(KeyCode::D) {
        rotation_velocity = -1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        rotation_velocity = 1.0;
    }

    tank_movement_intention.movement_velocity = movement_velocity;
    tank_movement_intention.rotation_velocity = rotation_velocity;
    if keyboard.just_pressed(KeyCode::Space) {
        tank_ammunition.fire_intent = true;
    }
}
