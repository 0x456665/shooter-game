use crate::{constant::*, player::Player};
use bevy::{asset::LoadedFolder, prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use rand::{Rng, seq::SliceRandom};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyBullet;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource)]
pub struct EnemyBulletTimer(pub Timer);

pub fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    game_asset: Res<GameAssets>,
    loaded_folders: Res<Assets<LoadedFolder>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    if let Some(enemies_folder) = loaded_folders.get(&game_asset.enemy_folder) {
        if enemies_folder.handles.is_empty() {
            return;
        }

        let mut rng = rand::rng();
        let rand_spawn = rng.random_range(MIN_ENEMY..=MAX_ENEMY);
        let random_index = rng.random_range(0..enemies_folder.handles.len());

        for _ in 0..rand_spawn {
            // Generate random x position avoiding player area
            let x = if rng.random_bool(0.5) {
                // Left side
                rng.random_range(-WINDOW_WIDTH / 2.0..=-PLAYER_SIZE.x * 2.0) + PLAYER_SIZE.x
            } else {
                // Right side
                rng.random_range(PLAYER_SIZE.x * 2.0..=WINDOW_WIDTH / 2.0) - PLAYER_SIZE.x
            };

            commands.spawn((
                RigidBody::Dynamic,
                GravityScale(0.0),
                Velocity {
                    linvel: Vec2::new(0.0, -PLAYER_SPEED * 0.8),
                    angvel: 0.0,
                },
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                Collider::cuboid(ENEMY_SIZE.x / 2.0, ENEMY_SIZE.y / 2.0),
                Transform::from_xyz(x, WINDOW_HEIGHT / 2.0, 0.0),
                Sprite {
                    image: enemies_folder.handles[random_index].clone().typed(),
                    custom_size: Some(ENEMY_SIZE),
                    anchor: Anchor::Center,
                    ..default()
                },
                Enemy,
            ));
        }
    }
}

pub fn cleanup_enemy(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, transform) in enemy_query.iter() {
        // Remove bullets that have gone off-screen
        if transform.translation.y > WINDOW_HEIGHT / 2.0 + 50.0 {
            commands.entity(entity).despawn();
        }
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 - 50.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn enemy_shoot(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut bullet_timer: ResMut<EnemyBulletTimer>,
) {
    if bullet_timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let enemies: Vec<_> = enemy_query.iter().collect();

        // Get player position - return early if no player exists
        let Ok(player_transform) = player_query.single() else {
            return;
        };
        let player_position = player_transform.translation.truncate(); // Convert Vec3 to Vec2

        if !enemies.is_empty() {
            // Only have 1-2 random enemies shoot each timer tick
            let num_shooters = rng.random_range(1..=3.min(enemies.len()));
            // Randomly select which enemies shoot
            let mut indices: Vec<usize> = (0..enemies.len()).collect();
            indices.shuffle(&mut rng);

            for i in 0..num_shooters {
                let enemy_transform = enemies[indices[i]];

                // Get enemy position (where the bullet will spawn from)
                let enemy_position = enemy_transform.translation.truncate();
                if enemy_position.y < -WINDOW_HEIGHT * 0.8 {
                    continue;
                }
                let bullet_spawn_position = enemy_position + Vec2::new(0.0, -ENEMY_SIZE.y / 2.0);

                // Calculate direction vector from enemy to player
                let direction = player_position - bullet_spawn_position;

                // Normalize the direction to get a unit vector
                let normalized_direction = direction.normalize();

                // Calculate bullet velocity by multiplying direction with desired speed
                let bullet_speed = BULLET_SPEED * 0.6; // Enemy bullets are slower than player bullets
                let bullet_velocity = normalized_direction * bullet_speed;

                //Add some inaccuracy to make it more interesting
                let inaccuracy = 20.0; // Adjust this value to control accuracy
                let random_offset = Vec2::new(
                    rng.random_range(-inaccuracy..=inaccuracy),
                    rng.random_range(-inaccuracy..=inaccuracy),
                );
                let final_velocity = bullet_velocity + random_offset;

                commands.spawn((
                    RigidBody::Dynamic,
                    Collider::cuboid(BULLET_SIZE.x / 2.0, BULLET_SIZE.y / 2.0),
                    Velocity {
                        linvel: final_velocity, // Use calculated velocity instead of fixed downward movement
                        angvel: 0.0,
                    },
                    Transform::from_translation(
                        enemy_transform.translation + Vec3::new(0.0, -ENEMY_SIZE.y / 2.0, 0.0),
                    ),
                    Sprite {
                        image: asset_server.load("PNG/Lasers/laserRed07.png"),
                        custom_size: Some(BULLET_SIZE),
                        anchor: Anchor::Center,
                        ..default()
                    },
                    GravityScale(0.0),
                    EnemyBullet,
                    Bullet,
                ));
            }

            // Play one sound effect per shooting round, not per bullet
            if num_shooters > 0 {
                commands.spawn(AudioPlayer::new(asset_server.load("Bonus/sfx_laser2.ogg")));
            }
        }
    }
}
