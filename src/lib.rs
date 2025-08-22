pub mod constant;
pub mod enemy;
pub mod player;

// use bevy::window::{PresentMode, WindowResolution};
// use bevy::{prelude::*, sprite::Anchor};

// fn main() {
//     App::new()
//         .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0))) // Black background\
//         .add_plugins(
//             DefaultPlugins.set(WindowPlugin {
//                 primary_window: Some(Window {
//                     title: "First Bevy Game!".into(),
//                     resolution: WindowResolution::new(WINDOW_HEIGHT, WINDOW_WIDTH)
//                         .with_scale_factor_override(1.0),
//                     present_mode: PresentMode::AutoVsync,
//                     // Tells wasm to resize the window according to the available canvas
//                     fit_canvas_to_parent: true,
//                     // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
//                     prevent_default_event_handling: false,
//                     ..default()
//                 }),
//                 ..default()
//             }),
//         )
//         .add_systems(Startup, initialize_camera)
//         .add_systems(Startup, spawn_simple_sprite)
//         .add_systems(Update, move_player)
//         .run();
// }
// const WINDOW_WIDTH: f32 = 1000.0;
// const WINDOW_HEIGHT: f32 = 1000.0;

// #[derive(Component)]
// #[require(Camera2d)]
// pub struct MainCamera;

// #[derive(Component)]
// struct Player {
//     velocity: u8,
// }

// #[derive(Component)]
// struct Enemy;

// #[derive(Component)]
// struct EnemyBullet;

// #[derive(Component)]
// struct PlayerBullet;

// const CUSTOM_SIZE: Vec2 = Vec2::new(40.0, 40.0);

// fn initialize_camera(mut commands: Commands) {
//     commands.spawn(MainCamera);
// }

// fn spawn_simple_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn((
//         Transform::default(),
//         Sprite {
//             image: asset_server.load("PNG/playerShip3_blue.png"),
//             custom_size: Some(CUSTOM_SIZE.clone()),
//             anchor: Anchor::BottomLeft,
//             ..default()
//         },
//         Player { velocity: 5 },
//     ));
// }

// fn move_player(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, (With<Player>, With<Sprite>)>,
// ) {
//     let mut player = query.single_mut().unwrap();
//     if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
//         if player.translation.y + 5.0 < WINDOW_HEIGHT {
//             player.translation.y += 5.0;
//         }
//     }
//     if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
//         if player.translation.y - 5.0 > -WINDOW_HEIGHT {
//             player.translation.y -= 5.0;
//         }
//     }
//     if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
//         if player.translation.x - 5.0 > -WINDOW_WIDTH / 2.0 {
//             player.translation.x -= 5.0;
//         }
//     }
//     if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
//         if player.translation.x + 5.0 < WINDOW_WIDTH / 2.0 {
//             player.translation.x += 5.0;
//         }
//     }
// }
