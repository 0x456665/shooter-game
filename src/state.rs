use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::constant::GameValues;

// Define your game states
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Settings,
    Loading,
}

// Marker components for different UI screens
#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct PauseMenuUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct SettingsUI;

// Systems for entering states (setup UI)
pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn main menu UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("SPACE SHOOTER"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future.ttf"),
                    font_size: 60.0,
                    ..Default::default()
                },
                TextColor(WHITE.into()),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Play button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PLAY"),
                        TextFont {
                            font: asset_server.load("Bonus/kenvector_future.ttf"),
                            font_size: 24.0,
                            ..Default::default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });

            // Settings button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
                    SettingsButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("SETTINGS"),
                        TextFont {
                            font: asset_server.load("Bonus/kenvector_future.ttf"),
                            font_size: 24.0,
                            ..Default::default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
        });
}

pub fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // HUD elements during gameplay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            GameUI,
        ))
        .with_children(|parent| {
            // Score display (top left)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Px(20.0),
                    ..default()
                },
                Text::new("Score: 0"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future.ttf"),
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(WHITE.into()),
                ScoreText,
            ));

            // Health display (top right)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    ..default()
                },
                Text::new("Health: 5"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future.ttf"),
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(WHITE.into()),
                HealthText,
            ));
        });
}

pub fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Semi-transparent overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // Semi-transparent black
            PauseMenuUI,
        ))
        .with_children(|parent| {
            // Pause menu container
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(400.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    // Paused title
                    parent.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font: asset_server.load("Bonus/kenvector_future.ttf"),
                            font_size: 36.0,
                            ..Default::default()
                        },
                        TextColor(WHITE.into()),
                        Node {
                            margin: UiRect::bottom(Val::Px(30.0)),
                            ..default()
                        },
                    ));

                    // Resume button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                            ResumeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("RESUME"),
                                TextFont {
                                    font: asset_server.load("Bonus/kenvector_future.ttf"),
                                    font_size: 18.0,
                                    ..Default::default()
                                },
                                TextColor(WHITE.into()),
                            ));
                        });

                    // Main Menu button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                            MainMenuButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("MAIN MENU"),
                                TextFont {
                                    font: asset_server.load("Bonus/kenvector_future.ttf"),
                                    font_size: 18.0,
                                    ..Default::default()
                                },
                                TextColor(WHITE.into()),
                            ));
                        });
                });
        });
}

pub fn setup_gameover_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            GameOverUI,
        ))
        .with_children(|parent| {
            // Play button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("REPLAY"),
                        TextFont {
                            font: asset_server.load("Bonus/kenvector_future.ttf"),
                            font_size: 24.0,
                            ..Default::default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });

            // Settings button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
                    MainMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("MAIN MENU"),
                        TextFont {
                            font: asset_server.load("Bonus/kenvector_future.ttf"),
                            font_size: 24.0,
                            ..Default::default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
        });
}

// Systems for exiting states (cleanup UI)
pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_game_ui(mut commands: Commands, query: Query<Entity, With<GameUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_gameover_ui(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Button marker components
#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct MainMenuButton;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthText;

// Button interaction systems
pub fn handle_play_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_values: ResMut<GameValues>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game_values.score = 0;
            game_values.health = 10;
            next_state.set(GameState::Playing);
        }
    }
}

pub fn handle_settings_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Settings);
        }
    }
}

pub fn handle_resume_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn handle_main_menu_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}

// Keyboard input for pausing
pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

// Update HUD systems
pub fn update_score_display(
    mut query: Query<&mut Text, With<ScoreText>>,
    game_values: Res<GameValues>, // Add your score resource here
) {
    for mut text in query.iter_mut() {
        // Update with actual score
        text.0 = format!("Score: {}", game_values.score); // Replace 0 with actual score
    }
}

pub fn update_health_display(
    mut query: Query<&mut Text, With<HealthText>>,
    game_values: Res<GameValues>,
    // Add your health resource here
) {
    for mut text in query.iter_mut() {
        // Update with actual health
        text.0 = format!("Health: {}", game_values.health); // Replace 5 with actual health
    }
}

// Example of how to register everything in main.rs
//
//
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default())
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(OnEnter(GameState::Playing), setup_game_ui)
            .add_systems(OnExit(GameState::Playing), cleanup_game_ui)
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_pause_menu)
            .add_systems(OnEnter(GameState::GameOver), setup_gameover_menu)
            .add_systems(OnExit(GameState::GameOver), cleanup_gameover_ui)
            .add_systems(
                Update,
                (
                    handle_play_button.run_if(in_state(GameState::MainMenu)),
                    handle_settings_button.run_if(in_state(GameState::MainMenu)),
                    handle_resume_button.run_if(in_state(GameState::Paused)),
                    handle_main_menu_button.run_if(in_state(GameState::Paused)),
                    handle_pause_input
                        .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
                    update_score_display.run_if(in_state(GameState::Playing)),
                    update_health_display.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}
