use std::sync::Mutex;

use crate::{
    enemy::{Enemy, EnemyBullet},
    player::{Player, PlayerBullet},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

static PLAYER_HEALTH: Mutex<u8> = Mutex::new(5);
static SCORE: Mutex<u16> = Mutex::new(0);

pub fn check_player_enemy_collisions(
    rapier_context: RapierContext,
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<Entity, (With<Enemy>, Without<Player>)>,
) {
    for (player_entity, _) in player_query.iter() {
        // Check if player is intersecting with any enemies
        for enemy_entity in enemy_query.iter() {
            if rapier_context.intersection_pair(player_entity, enemy_entity) == Some(true) {
                println!("Player is touching enemy!");
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
pub fn check_bullet_enemy_collisions(
    rapier_context: RapierContext,
    mut commands: Commands,
    player_bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    enemy_query: Query<Entity, (With<Enemy>, Without<Player>)>,
) {
    for (player_entity, _) in player_bullet_query.iter() {
        // Check if player is intersecting with any enemies
        for enemy_entity in enemy_query.iter() {
            if rapier_context.intersection_pair(player_entity, enemy_entity) == Some(true) {
                println!("Player is touching enemy!");
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
// fn check_bullet_player_collisions(
//     rapier_context: Res<RapierContext>,
//     player_query: Query<(Entity, &Transform), With<Player>>,
//     enemy_query: Query<Entity, (With<Enemy>, Without<Player>)>,
// ) {
//     for (player_entity, player_transform) in player_query.iter() {
//         // Check if player is intersecting with any enemies
//         for enemy_entity in enemy_query.iter() {
//             if rapier_context.intersection_pair(player_entity, enemy_entity) == Some(true) {
//                 println!("Player is touching enemy!");
//             }
//         }
//     }
// }

pub fn handle_collisions(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_bullet_query: Query<Entity, With<PlayerBullet>>,
    enemy_bullet_query: Query<Entity, With<EnemyBullet>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            // Get the player entity if it exists
            let player_entity = player_query.single().ok();

            // Player vs Enemy collision
            if let Some(player) = player_entity {
                let enemy_entity = if *entity1 == player && enemy_query.contains(*entity2) {
                    Some(*entity2)
                } else if *entity2 == player && enemy_query.contains(*entity1) {
                    Some(*entity1)
                } else {
                    None
                };

                if let Some(enemy) = enemy_entity {
                    println!("Player collided with enemy! Game Over!");
                    // Handle player death
                    if *PLAYER_HEALTH.lock().unwrap() == 0 {
                        commands.spawn(AudioPlayer::new(
                            asset_server.load("Bonus/sfx_lose.ogg"),
                        ));
                        commands.entity(player).despawn();
                    } else {
                        *PLAYER_HEALTH.lock().unwrap() -= 1;
                        commands.spawn(AudioPlayer::new(
                            asset_server.load("Bonus/sfx_zap.ogg"),
                        ));
                    }
                    commands.entity(enemy).despawn();
                    continue;
                }
            }

            // Player vs Enemy Bullet collision
            if let Some(player) = player_entity {
                let enemy_bullet = if *entity1 == player && enemy_bullet_query.contains(*entity2) {
                    Some(*entity2)
                } else if *entity2 == player && enemy_bullet_query.contains(*entity1) {
                    Some(*entity1)
                } else {
                    None
                };

                if let Some(bullet) = enemy_bullet {
                    println!("Player hit by enemy bullet! Game Over!");
                    // Handle player death
                    if *PLAYER_HEALTH.lock().unwrap() == 0 {
                        commands.spawn(AudioPlayer::new(
                            asset_server.load("Bonus/sfx_lose.ogg"),
                        ));
                        commands.entity(player).despawn();
                    } else {
                        *PLAYER_HEALTH.lock().unwrap() -= 1;
                        commands.spawn(AudioPlayer::new(
                            asset_server.load("Bonus/sfx_zap.ogg"),
                        ));
                    }
                    commands.entity(bullet).despawn();
                    continue;
                }
            }

            // Player Bullet vs Enemy collision
            let (bullet_entity, enemy_entity) =
                if player_bullet_query.contains(*entity1) && enemy_query.contains(*entity2) {
                    (Some(*entity1), Some(*entity2))
                } else if player_bullet_query.contains(*entity2) && enemy_query.contains(*entity1) {
                    (Some(*entity2), Some(*entity1))
                } else {
                    (None, None)
                };

            if let (Some(bullet), Some(enemy)) = (bullet_entity, enemy_entity) {
                println!("Player bullet hit enemy!");
                // Handle enemy destruction and scoring
                commands.entity(bullet).despawn();
                commands.entity(enemy).despawn();

                commands.spawn(AudioPlayer::new(
                    asset_server.load("Bonus/sfx_zap.ogg"),
                ));
                // TODO: Add score increment here
                *SCORE.lock().unwrap() += 1;
                continue;
            }

            // Player Bullet vs Enemy Bullet collision (optional - bullets destroy each other)
            let (player_bullet, enemy_bullet) = if player_bullet_query.contains(*entity1)
                && enemy_bullet_query.contains(*entity2)
            {
                (Some(*entity1), Some(*entity2))
            } else if player_bullet_query.contains(*entity2)
                && enemy_bullet_query.contains(*entity1)
            {
                (Some(*entity2), Some(*entity1))
            } else {
                (None, None)
            };

            if let (Some(p_bullet), Some(e_bullet)) = (player_bullet, enemy_bullet) {
                println!("Bullets collided and destroyed each other!");
                commands.entity(p_bullet).despawn();
                commands.entity(e_bullet).despawn();
            }
        }
    }
}
