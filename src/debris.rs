use crate::constant::*;
use bevy::{asset::LoadedFolder, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct DebrisTimer(pub Timer);

#[derive(Component)]
pub struct Debris;

pub fn spawn_debris(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<DebrisTimer>,
    game_asset: Res<GameAssets>,
    loaded_folders: Res<Assets<LoadedFolder>>,
) {
    // Only spawn if timer finished and debris folder is loaded
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    if let Some(debris_folder) = loaded_folders.get(&game_asset.debris_folder) {
        if debris_folder.handles.is_empty() {
            return; // No debris assets loaded yet
        }

        let mut rng = rand::rng();
        let debris_spawn: u8 = rng.random_range(0..=3);

        for _ in 0..debris_spawn {
            // Get random debris image
            let random_index = rng.random_range(0..debris_folder.handles.len());
            let random_handle = &debris_folder.handles[random_index];

            // Convert to typed image handle
            let image_handle = random_handle.clone();
            {
                // Choose random spawn side (0 = top, 1 = left, 2 = right)
                let spawn_side = rng.random_range(0..3);

                let (position, velocity) = match spawn_side {
                    0 => {
                        // Spawn from top
                        let x = rng.random_range(-WINDOW_WIDTH / 2.0..=WINDOW_WIDTH / 2.0);
                        let pos = Vec3::new(x, WINDOW_HEIGHT / 2.0 + 50.0, 0.0);
                        let vel = Vec2::new(
                            rng.random_range(-50.0..=50.0),    // Random horizontal drift
                            rng.random_range(-200.0..=-100.0), // Downward speed
                        );
                        (pos, vel)
                    }
                    1 => {
                        // Spawn from left
                        let y = rng.random_range(-WINDOW_HEIGHT / 2.0..=WINDOW_HEIGHT / 2.0);
                        let pos = Vec3::new(-WINDOW_WIDTH / 2.0 - 50.0, y, 0.0);
                        let vel = Vec2::new(
                            rng.random_range(100.0..=200.0), // Rightward speed
                            rng.random_range(-50.0..=50.0),  // Random vertical drift
                        );
                        (pos, vel)
                    }
                    _ => {
                        // Spawn from right
                        let y = rng.random_range(-WINDOW_HEIGHT / 2.0..=WINDOW_HEIGHT / 2.0);
                        let pos = Vec3::new(WINDOW_WIDTH / 2.0 + 50.0, y, 0.0);
                        let vel = Vec2::new(
                            rng.random_range(-200.0..=-100.0), // Leftward speed
                            rng.random_range(-50.0..=50.0),    // Random vertical drift
                        );
                        (pos, vel)
                    }
                };

                // Random size variation
                let scale = rng.random_range(0.5..=1.5);

                // Random rotation speed
                let angular_velocity = rng.random_range(-2.0..=2.0);

                commands.spawn((
                    RigidBody::KinematicVelocityBased,
                    Velocity {
                        linvel: velocity,
                        angvel: angular_velocity,
                    },
                    Collider::ball(32.0 * scale), // Adjust size based on your debris
                    ActiveEvents::COLLISION_EVENTS,
                    Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                    Sprite {
                        image: image_handle.typed(),
                        ..default()
                    },
                    Debris,
                ));
            }
        }
    }
}

// System to cleanup debris that goes off screen
pub fn cleanup_debris(
    mut commands: Commands,
    debris_query: Query<(Entity, &Transform), With<Debris>>,
) {
    for (entity, transform) in debris_query.iter() {
        let pos = transform.translation;

        // Remove debris if it goes too far off screen
        if pos.x < -WINDOW_WIDTH / 2.0 - 100.0
            || pos.x > WINDOW_WIDTH / 2.0 + 100.0
            || pos.y < -WINDOW_HEIGHT / 2.0 - 100.0
            || pos.y > WINDOW_HEIGHT / 2.0 + 100.0
        {
            commands.entity(entity).despawn();
        }
    }
}
