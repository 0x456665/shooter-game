use std::sync::Mutex;

use crate::{
    debris::Debris,
    enemy::{Enemy, EnemyBullet},
    player::{Player, PlayerBullet},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

static PLAYER_HEALTH: Mutex<u8> = Mutex::new(5);
static SCORE: Mutex<u16> = Mutex::new(0);

#[derive(Debug, Clone, Copy, PartialEq)]
enum EntityType {
    Player,
    Enemy,
    PlayerBullet,
    EnemyBullet,
    Debris,
    Unknown,
}

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


fn get_entity_type(
    entity: Entity,
    player_query: &Query<Entity, With<Player>>,
    enemy_query: &Query<Entity, With<Enemy>>,
    player_bullet_query: &Query<Entity, With<PlayerBullet>>,
    enemy_bullet_query: &Query<Entity, With<EnemyBullet>>,
    debris_query: &Query<Entity, With<Debris>>,
) -> EntityType {
    if player_query.contains(entity) {
        EntityType::Player
    } else if enemy_query.contains(entity) {
        EntityType::Enemy
    } else if player_bullet_query.contains(entity) {
        EntityType::PlayerBullet
    } else if enemy_bullet_query.contains(entity) {
        EntityType::EnemyBullet
    } else if debris_query.contains(entity) {
        EntityType::Debris
    } else {
        EntityType::Unknown
    }
}

fn handle_collision_and_despawn(
    entity1: Entity,
    entity2: Entity,
    type1: EntityType,
    type2: EntityType,
    commands: &mut Commands,
    asset_server: &ResMut<AssetServer>,
    despawned_entities: &mut std::collections::HashSet<Entity>,
) -> bool {
    use EntityType::*;
    
    match (type1, type2) {
        // Player vs Enemy (either order)
        (Player, Enemy) | (Enemy, Player) => {
            let (player, enemy) = if type1 == Player { (entity1, entity2) } else { (entity2, entity1) };
            println!("Player collided with enemy!");
            handle_player_damage(commands, asset_server, player, despawned_entities);
            despawn_entity(commands, enemy, despawned_entities);
            true
        }

        // Player vs Enemy Bullet (either order)
        (Player, EnemyBullet) | (EnemyBullet, Player) => {
            let (player, bullet) = if type1 == Player { (entity1, entity2) } else { (entity2, entity1) };
            println!("Player hit by enemy bullet!");
            handle_player_damage(commands, asset_server, player, despawned_entities);
            despawn_entity(commands, bullet, despawned_entities);
            true
        }

        // Any Bullet vs Debris (either order)
        (PlayerBullet, Debris) | (Debris, PlayerBullet) |
        (EnemyBullet, Debris) | (Debris, EnemyBullet) => {
            let bullet = if matches!(type1, PlayerBullet | EnemyBullet) { entity1 } else { entity2 };
            println!("Bullet hit debris!");
            despawn_entity(commands, bullet, despawned_entities);
            true
        }

        // Player Bullet vs Enemy (either order)
        (PlayerBullet, Enemy) | (Enemy, PlayerBullet) => {
            let (bullet, enemy) = if type1 == PlayerBullet { (entity1, entity2) } else { (entity2, entity1) };
            println!("Player bullet hit enemy!");
            despawn_entity(commands, enemy, despawned_entities);
            despawn_entity(commands, bullet, despawned_entities);
            commands.spawn(AudioPlayer::new(asset_server.load("Bonus/sfx_zap.ogg")));
            *SCORE.lock().unwrap() += 1;
            true
        }

        // Player Bullet vs Enemy Bullet (either order)
        (PlayerBullet, EnemyBullet) | (EnemyBullet, PlayerBullet) => {
            println!("Bullets collided and destroyed each other!");
            despawn_entity(commands, entity1, despawned_entities);
            despawn_entity(commands, entity2, despawned_entities);
            true
        }

        // No collision handling needed
        _ => false,
    }
}

fn handle_player_damage(
    commands: &mut Commands,
    asset_server: &ResMut<AssetServer>,
    player: Entity,
    despawned_entities: &mut std::collections::HashSet<Entity>,
) {
    let mut health = PLAYER_HEALTH.lock().unwrap();
    if *health <= 1 {
        *health = 0;
        commands.spawn(AudioPlayer::new(asset_server.load("Bonus/sfx_lose.ogg")));
        despawn_entity(commands, player, despawned_entities);
    } else {
        *health -= 1;
        commands.spawn(AudioPlayer::new(asset_server.load("Bonus/sfx_zap.ogg")));
    }
}

fn despawn_entity(
    commands: &mut Commands,
    entity: Entity,
    despawned_entities: &mut std::collections::HashSet<Entity>,
) {
    if !despawned_entities.contains(&entity) {
        commands.entity(entity).despawn();
        despawned_entities.insert(entity);
    }
}

pub fn handle_collisions(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    debris_query: Query<Entity, With<Debris>>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_bullet_query: Query<Entity, With<PlayerBullet>>,
    enemy_bullet_query: Query<Entity, With<EnemyBullet>>,
) {
    let mut despawned_entities = std::collections::HashSet::new();

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            // Skip if either entity has already been despawned this frame
            if despawned_entities.contains(entity1) || despawned_entities.contains(entity2) {
                continue;
            }

            let type1 = get_entity_type(*entity1, &player_query, &enemy_query, &player_bullet_query, &enemy_bullet_query, &debris_query);
            let type2 = get_entity_type(*entity2, &player_query, &enemy_query, &player_bullet_query, &enemy_bullet_query, &debris_query);

            handle_collision_and_despawn(
                *entity1,
                *entity2,
                type1,
                type2,
                &mut commands,
                &asset_server,
                &mut despawned_entities,
            );
        }
    }
}