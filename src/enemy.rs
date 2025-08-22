use crate::{constant::*, player::Velocity};
use bevy::{prelude::*, sprite::Anchor};
use rand::Rng;

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

pub fn enemy_shoot(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut bullet_timer: ResMut<EnemyBulletTimer>,
) {
    if bullet_timer.0.tick(time.delta()).just_finished() {
        for enemy in enemy_query {
            // Spawn bullet at player position
            commands.spawn((
                Transform::from_translation(
                    enemy.translation + Vec3::new(0.0, PLAYER_SIZE.y, 0.0),
                ),
                Sprite {
                    image: asset_server.load("PNG/Lasers/laserRed07.png"),
                    custom_size: Some(BULLET_SIZE),
                    anchor: Anchor::Center,
                    ..default()
                },
                EnemyBullet,
                Velocity(Vec2::new(0.0, BULLET_SPEED)),
            ));
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
