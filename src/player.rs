use super::constant::*;
use crate::enemy::EnemyBullet;
use bevy::{asset::LoadedFolder, prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct BulletTimer(pub Timer);

#[derive(Component)]
pub struct PlayerBullet;

#[derive(Resource, Default)]
pub struct PlayerSpawned(pub bool);

pub fn spawn_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut player_spawned: ResMut<PlayerSpawned>,
    existing_players: Query<&Player>,
    loaded_folder: Res<Assets<LoadedFolder>>,
) {
    if player_spawned.0 || !existing_players.is_empty() {
        return;
    }

    if let Some(player_folder) = loaded_folder.get(&game_assets.player_folder) {

        if player_folder.handles.is_empty() {
            return;
        }
        let mut rng = rand::rng();
        let random_index = rng.random_range(0..player_folder.handles.len());
        commands.spawn((
            RigidBody::KinematicPositionBased, // Better for manual movement
            ActiveEvents::COLLISION_EVENTS,
            Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0),
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + 100.0, 0.0),
            Sprite {
                image: player_folder.handles[random_index].clone().typed(),
                custom_size: Some(PLAYER_SIZE),
                anchor: Anchor::Center,
                ..default()
            },
            Sensor,
            Player,
        ));
        player_spawned.0 = true;
    } else {
        println!("Player folder not yet loaded, waiting...");
    }
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
    mouse_input: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut bullet_timer: ResMut<BulletTimer>,
) {
    if bullet_timer.0.tick(time.delta()).just_finished() {
        if keyboard_input.pressed(KeyCode::Space) || mouse_input.pressed(MouseButton::Left) {
            if let Ok(player_transform) = player_query.single() {
                // Spawn bullet at player position
                commands.spawn((
                    RigidBody::KinematicVelocityBased,
                    Collider::cuboid(BULLET_SIZE.x / 2.0, BULLET_SIZE.y / 2.0),
                    Transform::from_translation(
                        player_transform.translation + Vec3::new(0.0, PLAYER_SIZE.y / 2.0, 0.0),
                    ),
                    Velocity {
                        linvel: Vec2::new(0.0, BULLET_SPEED),
                        angvel: 0.0,
                    },
                    Sprite {
                        image: asset_server.load("PNG/Lasers/laserBlue01.png"),
                        custom_size: Some(BULLET_SIZE),
                        anchor: Anchor::Center,
                        ..default()
                    },
                    Sensor,
                    Bullet,
                    PlayerBullet,
                    ActiveEvents::COLLISION_EVENTS,
                ));

                commands.spawn(AudioPlayer::new(asset_server.load("Bonus/sfx_laser1.ogg")));
            }
        }
    }
}

pub fn cleanup_bullets(
    mut commands: Commands,
    player_bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    enemy_bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
) {
    // Clean up player bullets
    for (entity, transform) in player_bullet_query.iter() {
        if transform.translation.y > WINDOW_HEIGHT / 2.0 + 50.0 {
            commands.entity(entity).despawn();
        }
    }

    // Clean up enemy bullets
    for (entity, transform) in enemy_bullet_query.iter() {
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 - 50.0 {
            commands.entity(entity).despawn();
        }
    }
}
