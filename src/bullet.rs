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
    speed: f32,
    ttl: f32,
    bullet_image: Handle<Image>,
    commands: &mut Commands,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: transform.translation + transform.up() * 15.0,
                ..transform
            },
            texture: bullet_image,
            ..Default::default()
        },
        Bullet { speed },
        TemporaryObject::new(ttl),
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