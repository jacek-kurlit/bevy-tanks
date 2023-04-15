use bevy::prelude::*;

use crate::temporary::TemporaryObject;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bullets).add_system(shooting_handler);
    }
}

fn shooting_handler(
    mut commands: Commands,
    time: Res<Time>,
    mut tank_ammunition_query: Query<(&Transform, &mut TankAmmunitation)>,
) {
    for (transform, mut tank_ammunition) in tank_ammunition_query.iter_mut() {
        if tank_ammunition.cooldown_timer.just_finished() {
            if tank_ammunition.fire_intent {
                spawn_bullet(*transform, tank_ammunition.as_mut(), &mut commands);
                tank_ammunition.cooldown_timer.reset();
            }
        } else {
            tank_ammunition.cooldown_timer.tick(time.delta());
        }

        tank_ammunition.fire_intent = false;
    }
}

// TODO: bullets should use rapier too?
fn spawn_bullet(
    transform: Transform,
    tank_ammunition: &mut TankAmmunitation,
    commands: &mut Commands,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                //FIXME: magic number offset from tank barrage - probably depends on tank sprite size
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
    bullet_speed: f32,
    bullet_tll: f32,
    bullet_image: Handle<Image>,
    _damage: u32,
    cooldown_timer: Timer,
    pub fire_intent: bool,
}

const INIT_BULLET_SPEED: f32 = 150.0;
const INIT_BULLET_TLL: f32 = 2.0;
const INIT_BULLET_DMG: u32 = 25;
const INIT_COOLDOWN: f32 = 2.0;

impl TankAmmunitation {
    pub fn new(bullet_image: Handle<Image>) -> Self {
        let mut cooldown_timer = Timer::from_seconds(INIT_COOLDOWN, TimerMode::Repeating);
        // I don't know how to do this properly, but it seems to work as I want timer to be fished to allow player to shoot
        // asap from start cooldown_timer.tick(cooldown_timer.duration());
        TankAmmunitation {
            bullet_speed: INIT_BULLET_SPEED,
            bullet_tll: INIT_BULLET_TLL,
            bullet_image,
            cooldown_timer,
            _damage: INIT_BULLET_DMG,
            fire_intent: false,
        }
    }
}
