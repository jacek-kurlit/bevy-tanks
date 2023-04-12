use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{bullet::TankAmmunitation, tracks::Tracks};

#[derive(Bundle)]
pub struct TankBundle {
    #[bundle]
    pub sprite: SpriteBundle,
    pub tank: Tank,
    pub tracks: Tracks,
    pub tank_ammunition: TankAmmunitation,
}

impl TankBundle {
    pub fn new(
        translation: Vec3,
        tank_image: Handle<Image>,
        tracks_image: Handle<Image>,
        tank_ammunition_image: Handle<Image>,
        flip_y: bool,
    ) -> Self {
        TankBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                sprite: Sprite {
                    flip_y,
                    ..Default::default()
                },
                texture: tank_image,
                ..default()
            },
            tank: Tank::new(),
            tracks: Tracks::new(tracks_image),
            tank_ammunition: TankAmmunitation::new(tank_ammunition_image),
        }
    }
}

const INIT_MOVEMENT_SPEED: f32 = 100.0;
// 1 * PI/180.0 == 1 degree in radians
const INIT_ROTATION_SPEED: f32 = 70.0 * PI / 180.0;

#[derive(Component, Default)]
pub struct Tank {
    pub move_speed: f32,
    pub rotation_speed: f32,
    health: u32,
}

impl Tank {
    pub fn new() -> Self {
        Self {
            move_speed: INIT_MOVEMENT_SPEED,
            rotation_speed: INIT_ROTATION_SPEED,
            health: 100,
        }
    }
}
