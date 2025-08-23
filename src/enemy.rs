use crate::{constant::*, player::Velocity};
use bevy::{prelude::*, sprite::Anchor};
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
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let rand_spawn = rng.random_range(MIN_ENEMY..=MAX_ENEMY);

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
                Transform::from_xyz(x, WINDOW_HEIGHT / 2.0, 0.0),
                Sprite {
                    image: asset_server.load("PNG/Enemies/enemyRed1.png"),
                    custom_size: Some(ENEMY_SIZE),
                    anchor: Anchor::Center,
                    ..default()
                },
                Enemy,
            ));
        }
    }
}

pub fn move_enemy(mut query: Query<&mut Transform, With<Enemy>>) {
    for mut enemy in query.iter_mut() {
        enemy.translation.y -= 2.0
    }
}

pub fn cleanup_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), (With<Enemy>, With<Sprite>)>,
) {
    for (entity, transform) in bullet_query.iter() {
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
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut bullet_timer: ResMut<EnemyBulletTimer>,
) {
    if bullet_timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let enemies: Vec<_> = enemy_query.iter().collect();

        if !enemies.is_empty() {
            // Only have 1-2 random enemies shoot each timer tick
            let num_shooters = rng.random_range(1..=3.min(enemies.len()));

            // Randomly select which enemies shoot
            let mut indices: Vec<usize> = (0..enemies.len()).collect();
            indices.shuffle(&mut rng);

            for i in 0..num_shooters {
                let enemy_transform = enemies[indices[i]];

                commands.spawn((
                    Transform::from_translation(
                        enemy_transform.translation + Vec3::new(0.0, -ENEMY_SIZE.y / 2.0, 0.0),
                    ),
                    Sprite {
                        image: asset_server.load("PNG/Lasers/laserRed07.png"),
                        custom_size: Some(BULLET_SIZE),
                        anchor: Anchor::Center,
                        ..default()
                    },
                    EnemyBullet,
                    Velocity(Vec2::new(0.0, BULLET_SPEED * 0.7)), // Slower enemy bullets
                ));
            }

            // Play one sound effect per shooting round, not per bullet
            if num_shooters > 0 {
                commands.spawn(AudioPlayer::new(asset_server.load("Bonus/sfx_laser2.ogg")));
            }
        }
    }
}

pub fn move_enemy_bullet(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<EnemyBullet>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y -= velocity.0.y * time.delta_secs();
    }
}
