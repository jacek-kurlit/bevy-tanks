use std::f32::consts::PI;

use crate::{bullet::spawn_bullet, tracks::Tracks};
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
    commands.insert_resource(Textures {
        bullet: asset_server.load("single_sprites/bulletBlue2_outline.png"),
    });
    let tank_transform = Transform {
        translation: Vec3::new(window.width() / 2.0, 50.0, 0.0),
        ..Default::default()
    };
    commands.spawn((
        SpriteBundle {
            transform: tank_transform,
            sprite: Sprite {
                flip_y: true,
                ..Default::default()
            },
            texture: asset_server.load("single_sprites/tank_blue.png"),
            ..default()
        },
        Player,
        Tank {
            current_transform: tank_transform,
            move_speed: PLAYER_MOVEMENT_SPEED,
            rotation_speed: PLAYER_ROTATION_SPEED,
            tracks: Tracks::new(asset_server.load("single_sprites/tracksSmall.png")),
        },
    ));
}

#[derive(Component)]
struct Player;
const PLAYER_MOVEMENT_SPEED: f32 = 100.0;
// 1 * PI/180.0 == 1 degree in radians
const PLAYER_ROTATION_SPEED: f32 = 70.0 * PI / 180.0;
fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut Tank), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    textures: Res<Textures>,
) {
    let (mut player_transform, mut tank) = player_query.single_mut();
    if keyboard.pressed(KeyCode::W) {
        let new_position = tank.move_to_new_position(player_transform.up(), &time, &mut commands);
        player_transform.translation = new_position;
    }
    if keyboard.pressed(KeyCode::S) {
        let new_position = tank.move_to_new_position(player_transform.down(), &time, &mut commands);
        player_transform.translation = new_position;
    }
    if keyboard.pressed(KeyCode::D) {
        // player_transform.rotate_z(-PLAYER_ROTATION_SPEED * time.delta_seconds());
        let new_rotation = tank.rotate_vehicle(-1.0, &time);
        player_transform.rotation = new_rotation;
    }
    if keyboard.pressed(KeyCode::A) {
        let new_rotation = tank.rotate_vehicle(1.0, &time);
        player_transform.rotation = new_rotation;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        spawn_bullet(
            *player_transform,
            //FIXME: depends on player upgrades
            150.0,
            2.0,
            textures.bullet.clone(),
            &mut commands,
        );
    }
}

#[derive(Resource)]
struct Textures {
    bullet: Handle<Image>,
}

#[derive(Component, Default)]
struct Tank {
    current_transform: Transform,
    move_speed: f32,
    rotation_speed: f32,
    tracks: Tracks,
}

impl Tank {
    fn move_to_new_position(
        &mut self,
        direction: Vec3,
        time: &Res<Time>,
        commands: &mut Commands,
    ) -> Vec3 {
        self.current_transform.translation += direction * self.move_speed * time.delta_seconds();
        self.tracks.update_tracks(&self.current_transform, commands);
        self.current_transform.translation
    }

    fn rotate_vehicle(&mut self, directon: f32, time: &Res<Time>) -> Quat {
        self.current_transform
            .rotate_z(directon * self.rotation_speed * time.delta_seconds());
        self.current_transform.rotation
    }
}
