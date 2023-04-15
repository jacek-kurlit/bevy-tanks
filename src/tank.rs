use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

use crate::{bullet::TankAmmunitation, tracks::Tracks};

pub struct TankPlugin;
impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(hanle_tank_movement);
    }
}

#[derive(Bundle)]
pub struct TankBundle {
    #[bundle]
    pub sprite: SpriteBundle,
    pub tank: Tank,
    pub tank_move_intention: TankMovementIntention,
    pub tracks: Tracks,
    pub tank_ammunition: TankAmmunitation,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub gavity_scale: GravityScale,
    pub collider: Collider,
    pub restitution: Restitution,
    pub collider_mass_properties: ColliderMassProperties,
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
            tank_move_intention: TankMovementIntention::default(),
            tracks: Tracks::new(tracks_image),
            tank_ammunition: TankAmmunitation::new(tank_ammunition_image),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            gavity_scale: GravityScale(0.0),
            // FIXME: this depends on tank image size
            collider: Collider::cuboid(25.0, 25.0),
            restitution: Restitution::coefficient(0.0),
            collider_mass_properties: ColliderMassProperties::Mass(2000.0),
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
    _health: u32,
}

impl Tank {
    pub fn new() -> Self {
        Self {
            move_speed: INIT_MOVEMENT_SPEED,
            rotation_speed: INIT_ROTATION_SPEED,
            _health: 100,
        }
    }
}

#[derive(Component, Default)]
pub struct TankMovementIntention {
    pub movement_velocity: Vec2,
    pub rotation_velocity: f32,
}

impl TankMovementIntention {
    fn tank_moved(&self) -> bool {
        self.movement_velocity.length() > 0.0
    }
}

fn hanle_tank_movement(
    mut tank_query: Query<(
        &mut Transform,
        &Tank,
        &mut Tracks,
        &mut Velocity,
        &mut TankMovementIntention,
    )>,
    mut commands: Commands,
) {
    for (transform, tank, mut tracks, mut velocity, movement_intention) in tank_query.iter_mut() {
        velocity.linvel = movement_intention.movement_velocity * tank.move_speed;
        velocity.angvel = movement_intention.rotation_velocity * tank.rotation_speed;
        if movement_intention.tank_moved() {
            tracks.update_tracks(&transform, &mut commands);
        }
    }
}
