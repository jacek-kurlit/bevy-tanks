use bevy::prelude::*;

pub struct TemporaryObjectsPlugin;

impl Plugin for TemporaryObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_temporary_objects);
    }
}
#[derive(Component)]
pub struct TemporaryObject {
    timer: Timer,
}

impl TemporaryObject {
    pub fn new(duration_seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_seconds, TimerMode::Once),
        }
    }
}

fn handle_temporary_objects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut TemporaryObject, Entity)>,
) {
    for (mut temporary_object, entity) in query.iter_mut() {
        temporary_object.timer.tick(time.delta());
        if temporary_object.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
