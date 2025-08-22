use crate::enemy::EnemyBullet;

use super::constant::*;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct BulletTimer(pub Timer);

#[derive(Component)]
pub struct PlayerBullet;

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + 100.0, 0.0), // Start near bottom
        Sprite {
            image: asset_server.load("PNG/playerShip3_blue.png"),
            custom_size: Some(PLAYER_SIZE),
            anchor: Anchor::Center, // Center anchor is usually better for games
            ..default()
        },
        Player,
    ));
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = Vec2::ZERO;

        // Calculate movement direction
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // Normalize diagonal movement and apply speed
        if direction != Vec2::ZERO {
            direction = direction.normalize();
            let movement = direction * PLAYER_SPEED * time.delta_secs();
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;

            // Keep player within screen bounds
            transform.translation.x = transform.translation.x.clamp(
                -WINDOW_WIDTH / 2.0 + PLAYER_SIZE.x / 2.0,
                WINDOW_WIDTH / 2.0 - PLAYER_SIZE.x / 2.0,
            );
            transform.translation.y = transform.translation.y.clamp(
                -WINDOW_HEIGHT / 2.0 + PLAYER_SIZE.y / 2.0,
                WINDOW_HEIGHT / 2.0 - PLAYER_SIZE.y / 2.0,
            );
        }
    }
}

pub fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut bullet_timer: ResMut<BulletTimer>,
) {
    if bullet_timer.0.tick(time.delta()).just_finished() {
        if keyboard_input.pressed(KeyCode::Space) {
            if let Ok(player_transform) = player_query.single() {
                // Spawn bullet at player position
                commands.spawn((
                    Transform::from_translation(
                        player_transform.translation + Vec3::new(0.0, PLAYER_SIZE.y / 2.0, 0.0),
                    ),
                    Sprite {
                        image: asset_server.load("PNG/Lasers/laserBlue01.png"),
                        custom_size: Some(BULLET_SIZE),
                        anchor: Anchor::Center,
                        ..default()
                    },
                    PlayerBullet,
                    Velocity(Vec2::new(0.0, BULLET_SPEED)),
                ));
            }
        }
    }
}

pub fn move_bullet(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<PlayerBullet>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

pub fn cleanup_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), (With<PlayerBullet>, With<EnemyBullet>)>,
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
