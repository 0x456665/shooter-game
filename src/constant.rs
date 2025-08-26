use bevy::asset::LoadedFolder;
use bevy::math::Vec2;
use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
pub const PLAYER_SPEED: f32 = 250.0; // pixels per second
pub const BULLET_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 40.0);
pub const BULLET_SIZE: Vec2 = Vec2::new(4.0, 15.0);
pub const MAX_ENEMY: u8 = 4;
pub const MIN_ENEMY: u8 = 1;
pub const ENEMY_SIZE: Vec2 = Vec2::new(40.0, 40.0);

#[derive(Resource, Default)]
pub struct GameAssets {
    pub enemy_folder: Handle<LoadedFolder>,
    pub debris_folder: Handle<LoadedFolder>,
    pub player_folder: Handle<LoadedFolder>,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct GameValues {
    pub score: u16,
    pub health: u8,
}
