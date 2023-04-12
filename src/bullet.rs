use bevy::prelude::*;

use crate::temporary::TemporaryObject;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bullets);
    }
}

pub fn spawn_bullet(
    transform: Transform,
    tank_ammunition: &TankAmmunitation,
    commands: &mut Commands,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: transform.translation + transform.up() * 15.0,
                ..transform
            },
            texture: tank_ammunition.bullet_image.clone(),
            ..Default::default()
        },
        Bullet {
            speed: tank_ammunition.bullet_speed,
        },
        TemporaryObject::new(tank_ammunition.bullet_tll),
    ));
}

#[derive(Component)]
struct Bullet {
    speed: f32,
}

fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut bullet_transform, bullet) in bullet_query.iter_mut() {
        let forward = bullet_transform.up() * bullet.speed * time.delta_seconds();
        bullet_transform.translation += forward;
    }
}

#[derive(Component)]
pub struct TankAmmunitation {
    pub bullet_speed: f32,
    pub bullet_tll: f32,
    pub bullet_image: Handle<Image>,
    pub damage: u32,
}

const INIT_BULLET_SPEED: f32 = 150.0;
const INIT_BULLET_TLL: f32 = 2.0;
const INIT_BULLET_DMG: u32 = 25;

impl TankAmmunitation {
    pub fn new(bullet_image: Handle<Image>) -> Self {
        TankAmmunitation {
            bullet_speed: INIT_BULLET_SPEED,
            bullet_tll: INIT_BULLET_TLL,
            bullet_image,
            damage: INIT_BULLET_DMG,
        }
    }
}
