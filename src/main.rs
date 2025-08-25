use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_rapier2d::prelude::*;
use first_bevy_game::collision::*;
use first_bevy_game::debris::*;
use first_bevy_game::{constant::*, enemy::*, player::*};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "First Bevy Game!".into(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .with_scale_factor_override(1.0),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .insert_resource(PlayerSpawned::default()) // Black background
        .insert_resource(BulletTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            2.,
            TimerMode::Repeating,
        )))
        .insert_resource(EnemyBulletTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .insert_resource(DebrisTimer(Timer::from_seconds(2., TimerMode::Repeating)))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                load_assets_once,
                add_background,
                initialize_camera,
            ),
        )
        .add_systems(
            Update,
            (
                spawn_player,
                move_player,
                player_shoot,
                cleanup_bullets,
                enemy_shoot,
                spawn_enemy.after(load_assets_once),
                handle_collisions,
                cleanup_enemy,
                handle_collisions,
                spawn_debris,
                cleanup_debris,
            ),
        )
        // .add_systems(Update, (move_bullet, move_enemy_bullet))
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
        image: asset_server.load("Backgrounds/darkPurple.png"),
        custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
        ..default()
    });
}

fn load_assets_once(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        player_folder: asset_server.load_folder("PNG/Players/"),
        debris_folder: asset_server.load_folder("PNG/Meteors/"),
        enemy_folder: asset_server.load_folder("PNG/Enemies/"),
    });
}
