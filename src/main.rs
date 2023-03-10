// std namespace uses
use std::f32::consts::PI;

// bevy namespace uses
use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Window constants
pub const ASPECT_RATIO: f32 = 16.0/9.0;
pub const WINDOW_HEIGHT: f32 =  720.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT*ASPECT_RATIO;

// Racket constants
pub const RACKET_HEIGHT: f32 = 100.0;
pub const RACKET_WIDTH: f32 = 20.0;
pub const RACKET_EDGE_OFFSET: f32 = 50.0;
pub const RACKET_SPEED: f32 = 400.0;

// Ball constants
pub const BALL_SIZE: f32 = 5.0;
pub const BALL_SPEED: f32 = 500.0;

// Other constants
pub const SCATTER_FACTOR: f32 = 0.3;
pub const TEXT_SIZE: f32 = 32.0;

////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Racket;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Ball {
    direction: f32,
    moving: bool,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player {
    player_number: u8,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Scoreboard {
    player_scores: [u8; 2],
}

////////////////////////////////////////////////////////////////
// App
////////////////////////////////////////////////////////////////
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .add_plugins(DefaultPlugins.set(
        WindowPlugin { 
            primary_window: Some(Window {
                title: "Bevy Pong Implementation".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }
    ))
    // .add_plugin(WorldInspectorPlugin)
    .register_type::<Racket>()
    .register_type::<Ball>()
    .register_type::<Player>()
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_rackets)
    .add_startup_system(spawn_ball)
    .add_startup_system(spawn_scoreboard)
    .add_system(serve_ball)
    .add_systems((
        control_rackets,
        bounce_ball,
        move_ball
    ).chain())
    .add_system(score_goal)
    .run();
}

////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////

// Camera systems
fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default()).insert(Name::new("Camera"));
}


// Racket systems
fn spawn_rackets(
    mut commands: Commands,
) {
    // Spawn player 1 (left) racket
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(RACKET_WIDTH, RACKET_HEIGHT)),
            ..default()
        },
        transform: Transform::from_xyz(-0.5*WINDOW_WIDTH+RACKET_EDGE_OFFSET, 0.0, 0.0),
        ..default()
    })
    .insert(Name::new("Player 1 Racket"))
    .insert(Racket)
    .insert(Player {player_number: 1});

    // Spawn player 2 (right) racket
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(RACKET_WIDTH, RACKET_HEIGHT)),
            ..default()
        },
        transform: Transform::from_xyz(0.5*WINDOW_WIDTH-RACKET_EDGE_OFFSET, 0.0, 0.0),
        ..default()
    })
    .insert(Name::new("Player 2 Racket"))
    .insert(Racket)
    .insert(Player {player_number: 2});
}

fn control_rackets(
    time: Res<Time>,
    mut rackets: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::W) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 1 && transform.translation.y < 0.5*WINDOW_HEIGHT-0.5*RACKET_HEIGHT {
                transform.translation.y += RACKET_SPEED*time.delta_seconds();
            }
        }
    }
    
    if keyboard.pressed(KeyCode::S) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 1 && transform.translation.y > -0.5*WINDOW_HEIGHT+0.5*RACKET_HEIGHT {
                transform.translation.y -= RACKET_SPEED*time.delta_seconds();
            }
        }
    }
    
    if keyboard.pressed(KeyCode::Up) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 2 && transform.translation.y < 0.5*WINDOW_HEIGHT-0.5*RACKET_HEIGHT {
                transform.translation.y += RACKET_SPEED*time.delta_seconds();
            }
        }
    }
    
    if keyboard.pressed(KeyCode::Down) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 2 && transform.translation.y > -0.5*WINDOW_HEIGHT+0.5*RACKET_HEIGHT {
                transform.translation.y -= RACKET_SPEED*time.delta_seconds();
            }
        }
    }
}


// Ball systems
fn spawn_ball(
    mut commands: Commands,
) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..default()
        },
        ..default()
    })
    .insert(Name::new("Ball"))
    .insert(Ball {
        direction: 0.5*PI,
        moving: false,
    });
}

fn serve_ball(
    mut balls: Query<&mut Ball>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let mut ball = balls.single_mut();  // There should only be one ball
        ball.moving = true;
    }
}

fn bounce_ball(
    mut balls: Query<(&mut Ball, &Transform)>,
    rackets: Query<(&Player, &Transform)>
) {
    let (mut ball, ball_transform) = balls.single_mut();  // There should only be one ball
    // Wall reflect
    if ball_transform.translation.y.abs() >= 0.5*WINDOW_HEIGHT {
        ball.direction = PI - ball.direction;
    }
    
    // Racket reflect
    for (player, racket_transform) in rackets.iter() {
        if player.player_number == 1                            // If player 1
           && modulus_f32(ball.direction, 2.0*PI) >= PI         // and ball is moving left
           && are_colliding(ball_transform, racket_transform)   // and ball is colliding with racket
        {
            let scatter: f32 = SCATTER_FACTOR * (ball_transform.translation.y - racket_transform.translation.y)/RACKET_HEIGHT * PI;  // Calculate scatter angle
            ball.direction = 2.0*PI - ball.direction - scatter;  // redirect ball
        }

        if player.player_number == 2                           // If player 2
           && modulus_f32(ball.direction, 2.0*PI) <= PI        // and ball is moving right
           && are_colliding(ball_transform, racket_transform)  // and ball is colliding with racket
        {
            let scatter: f32 = SCATTER_FACTOR * (ball_transform.translation.y - racket_transform.translation.y)/RACKET_HEIGHT * PI;  // Calculate scatter angle
            ball.direction = 2.0*PI - ball.direction + scatter;  // redirect ball
        }
    }
}

fn move_ball(
    time: Res<Time>,
    mut balls: Query<(&Ball, &mut Transform)>
) {
    let (ball, mut ball_transform) = balls.single_mut();  // There should only be one ball
    if ball.moving {
        ball_transform.translation.x += (ball.direction.sin())*BALL_SPEED*time.delta_seconds();
        ball_transform.translation.y += (ball.direction.cos())*BALL_SPEED*time.delta_seconds();
    }
}

fn score_goal(
    mut balls: Query<(&mut Ball, &mut Transform)>,
    mut scoreboards: Query<(&mut Scoreboard, &mut Text)>
) {
    let (mut ball, mut ball_transform) = balls.single_mut();  // There should only be one ball
    let (mut scoreboard, mut scoreboard_text) = scoreboards.single_mut();  // There should only be one scoreboard
    
    // If ball hits left side of screen
    if ball_transform.translation.x <= -0.5*WINDOW_WIDTH {
        // Return to center, player 2 serve
        ball.direction = 0.5*PI;
        ball.moving = false;
        ball_transform.translation.x = 0.0;
        ball_transform.translation.y = 0.0;
        
        scoreboard.player_scores[1] += 1;

        scoreboard_text.sections[0].value = scoreboard.player_scores[0].to_string();
        scoreboard_text.sections[2].value = scoreboard.player_scores[1].to_string();
    }
    
    // If ball hits right side of screen
    if ball_transform.translation.x >= 0.5*WINDOW_WIDTH {
        // Return to center, player 1 serve
        ball.direction = -0.5*PI;
        ball.moving = false;
        ball_transform.translation.x = 0.0;
        ball_transform.translation.y = 0.0;
        
        scoreboard.player_scores[0] += 1;

        scoreboard_text.sections[0].value = scoreboard.player_scores[0].to_string();
        scoreboard_text.sections[2].value = scoreboard.player_scores[1].to_string();
    }
}


// Scoreboard systems
fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    
    let text_style = TextStyle {
        font: asset_server.load("fonts/consolas.ttf"),
        font_size: TEXT_SIZE,
        color: Color::WHITE,
    };
    
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection::new("0", text_style.clone()),
                TextSection::new(" : ", text_style.clone()),
                TextSection::new("0", text_style.clone()),
            ],
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.5*WINDOW_HEIGHT-0.75*TEXT_SIZE, 0.0),
        ..default()
    })
    .insert(Scoreboard {
        player_scores: [0,0]
    });

    
}

////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////

fn modulus_f32(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

fn are_colliding(ball_transform: &Transform, racket_transform: &Transform) -> bool {
       ball_transform.translation.x >= racket_transform.translation.x - 0.5*RACKET_WIDTH - 0.5*BALL_SIZE
    && ball_transform.translation.x <= racket_transform.translation.x + 0.5*RACKET_WIDTH + 0.5*BALL_SIZE
    && ball_transform.translation.y >= racket_transform.translation.y - 0.5*RACKET_HEIGHT - 0.5*BALL_SIZE
    && ball_transform.translation.y <= racket_transform.translation.y + 0.5*RACKET_HEIGHT + 0.5*BALL_SIZE
}