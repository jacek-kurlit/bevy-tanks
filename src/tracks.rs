use crate::temporary::TemporaryObject;
use bevy::prelude::*;

pub struct Tracks {
    tracks_sprite: Handle<Image>,
    last_track_position: Vec3,
    new_tracks_distance: f32,
    ttl: f32,
}

impl Tracks {
    pub fn new(tracks_sprite: Handle<Image>) -> Self {
        Self {
            tracks_sprite,
            last_track_position: Default::default(),
            ..Default::default()
        }
    }

    pub fn update_tracks(&mut self, new_position: &Transform, commands: &mut Commands) {
        let distance = new_position.translation.distance(self.last_track_position);
        if distance > self.new_tracks_distance {
            let tb = TrackBundle::new(*new_position, self.ttl, self.tracks_sprite.clone());
            self.last_track_position = tb.sprite.transform.translation;
            commands.spawn(tb);
        }
    }
}

#[derive(Bundle, Default)]
struct TrackBundle {
    pub temporary_object: TemporaryObject,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl TrackBundle {
    fn new(transform: Transform, ttl: f32, texture: Handle<Image>) -> Self {
        let tracks_position = transform.translation + transform.down() * 10.0;
        Self {
            temporary_object: TemporaryObject::new(ttl),
            sprite: SpriteBundle {
                transform: Transform {
                    translation: tracks_position,
                    ..transform
                },
                texture,
                ..Default::default()
            },
        }
    }
}

impl Default for Tracks {
    fn default() -> Self {
        Self {
            tracks_sprite: Handle::default(),
            last_track_position: Vec3::default(),
            new_tracks_distance: 66.0,
            ttl: 2.0,
        }
    }
}
