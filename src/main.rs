use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_input)
        .add_system(handle_bullets)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
        ..default()
    });
    println!("window width {} height {}", window.width(), window.height());
    let tank_position = Vec3::new(window.width() / 2.0, 50.0, 0.0);
    // let tank_barrel = commands
    //     .spawn(SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(0.0, -10.0, 0.0),
    //             ..default()
    //         },
    //         texture: asset_server.load("single_sprites/tankBlue_barrel2_outline.png"),
    //         ..Default::default()
    //     })
    //     .id();

    let player_tank = asset_server.load("single_sprites/tank_blue.png");
    commands.insert_resource(Textures {
        _player_tank: player_tank.clone(),
        bullet: asset_server.load("single_sprites/bulletBlue2_outline.png"),
    });
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: tank_position,
                ..Default::default()
            },
            sprite: Sprite {
                flip_y: true,
                ..Default::default()
            },
            texture: player_tank,
            ..default()
        },
        Player,
    ));
    // .push_children(&[tank_barrel]);
}

#[derive(Component)]
struct Player;
const PLAYER_MOVEMENT_SPEED: f32 = 100.0;
// 1 * PI/180.0 == 1 degree in radians
const PLAYER_ROTATION_SPEED: f32 = 70.0 * PI / 180.0;
fn player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    textures: Res<Textures>,
) {
    let mut player_transform = player_query.single_mut();
    if keyboard.pressed(KeyCode::W) {
        let forward = player_transform.up() * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
        player_transform.translation += forward;
    }
    if keyboard.pressed(KeyCode::S) {
        let backward = player_transform.down() * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
        player_transform.translation += backward;
    }
    if keyboard.pressed(KeyCode::D) {
        player_transform.rotate_z(-PLAYER_ROTATION_SPEED * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::A) {
        player_transform.rotate_z(PLAYER_ROTATION_SPEED * time.delta_seconds());
    }
    if keyboard.just_pressed(KeyCode::Space) {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: player_transform.translation + player_transform.up() * 10.0,
                    ..*player_transform
                },
                texture: textures.bullet.clone(),
                ..Default::default()
            },
            Bullet::new(),
        ));
    }
}

const BULLET_SPEED: f32 = 150.0;

fn handle_bullets(
    mut bullet_query: Query<(&mut Transform, &mut Bullet, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut bullet_transform, mut bullet, entity) in bullet_query.iter_mut() {
        let forward = bullet_transform.up() * BULLET_SPEED * time.delta_seconds();
        bullet_transform.translation += forward;
        bullet.timer.tick(time.delta());
        if bullet.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Resource)]
struct Textures {
    _player_tank: Handle<Image>,
    bullet: Handle<Image>,
}

#[derive(Component)]
struct Bullet {
    timer: Timer,
}

impl Bullet {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}
