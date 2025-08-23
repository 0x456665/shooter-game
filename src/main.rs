use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use first_bevy_game::{constant::*, enemy::*, player::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0))) // Black background
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "First Bevy Game!".into(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .with_scale_factor_override(1.0),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(BulletTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            2.,
            TimerMode::Repeating,
        )))
        .insert_resource(EnemyBulletTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, add_background)
        .add_systems(Startup, (initialize_camera, spawn_player))
        .add_systems(Update, (move_player, move_enemy))
        .add_systems(Update, (player_shoot, cleanup_bullets, enemy_shoot))
        .add_systems(Update, (move_bullet, move_enemy_bullet))
        .add_systems(Update, (spawn_enemy, cleanup_enemy))
        .run();
}

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

fn initialize_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn add_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite {
        image: asset_server.load("Backgrounds/black.png"),
        custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
        ..default()
    });
}
