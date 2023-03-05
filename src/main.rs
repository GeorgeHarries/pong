use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


pub const ASPECT_RATIO: f32 = 16.0/9.0;
pub const WINDOW_HEIGHT: f32 =  720.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT*ASPECT_RATIO;

pub const RACKET_HEIGHT: f32 = 100.0;
pub const RACKET_WIDTH: f32 = 10.0;
pub const RACKET_EDGE_OFFSET: f32 = 50.0;
pub const RACKET_SPEED: f32 = 10.0;

pub const BALL_SIZE: f32 = 5.0;
pub const BALL_SPEED: f32 = 10.0;

pub const SCATTER_FACTOR: f32 = 0.3;

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
    player_number: u8
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(
            WindowPlugin { 
                window: WindowDescriptor{
                    width: WINDOW_WIDTH,
                    height: WINDOW_HEIGHT,
                    title: "Bevy Pong Implementation".to_string(),
                    resizable: false,
                    ..Default::default()
                },
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
        .add_system(control_rackets)
        .add_system(serve_ball)
        .add_system(bounce_ball)
        .add_system(move_ball)
        .add_system(score_goal)
        .run();
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default()).insert(Name::new("Camera"));
}



fn spawn_rackets(
    mut commands: Commands,
) {
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
    mut rackets: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::W) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 1 && transform.translation.y < 0.5*WINDOW_HEIGHT-0.5*RACKET_HEIGHT {
                transform.translation.y += RACKET_SPEED;
            }
        }
    }
    
    if keyboard.pressed(KeyCode::S) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 1 && transform.translation.y > -0.5*WINDOW_HEIGHT+0.5*RACKET_HEIGHT {
                transform.translation.y -= RACKET_SPEED;
            }
        }
    }
    
    if keyboard.pressed(KeyCode::Up) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 2 && transform.translation.y < 0.5*WINDOW_HEIGHT-0.5*RACKET_HEIGHT {
                transform.translation.y += RACKET_SPEED;
            }
        }
    }
    
    if keyboard.pressed(KeyCode::Down) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 2 && transform.translation.y > -0.5*WINDOW_HEIGHT+0.5*RACKET_HEIGHT {
                transform.translation.y -= RACKET_SPEED;
            }
        }
    }
}



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
        direction: 0.5*std::f32::consts::PI,
        moving: false,
    });
}

fn serve_ball(
    mut balls: Query<&mut Ball>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut ball in balls.iter_mut(){
            ball.moving = true;
        }
    }
}

fn bounce_ball(
    mut balls: Query<(&mut Ball, &Transform)>,
    rackets: Query<(&Player, &Transform)>
) {
    for (mut ball, ball_transform) in balls.iter_mut() {
        // Wall reflect
        if ball_transform.translation.y.abs() >= 0.5*WINDOW_HEIGHT {
            ball.direction = std::f32::consts::PI - ball.direction;
        }
        
        // Racket reflect TODO: check y pos
        for (player, racket_transform) in rackets.iter() {
            if player.player_number == 1
               && ball_transform.translation.x <= racket_transform.translation.x + 0.5*RACKET_WIDTH
               && ball_transform.translation.x >= racket_transform.translation.x
               && ball_transform.translation.y > racket_transform.translation.y - 0.5*RACKET_HEIGHT
               && ball_transform.translation.y < racket_transform.translation.y + 0.5*RACKET_HEIGHT
            {
                let scatter: f32 = SCATTER_FACTOR * (ball_transform.translation.y - racket_transform.translation.y)/RACKET_HEIGHT * std::f32::consts::PI;
                ball.direction = 2.0*std::f32::consts::PI - ball.direction + scatter;
            }
            if player.player_number == 2
               && ball_transform.translation.x >= racket_transform.translation.x - 0.5*RACKET_WIDTH
               && ball_transform.translation.x >= racket_transform.translation.x
               && ball_transform.translation.y > racket_transform.translation.y - 0.5*RACKET_HEIGHT
               && ball_transform.translation.y < racket_transform.translation.y + 0.5*RACKET_HEIGHT
            {
                let scatter: f32 = SCATTER_FACTOR * (ball_transform.translation.y - racket_transform.translation.y)/RACKET_HEIGHT * std::f32::consts::PI;
                ball.direction = 2.0*std::f32::consts::PI - ball.direction + scatter;
            }
        }

    }
}

fn move_ball(
    mut balls: Query<(&Ball, &mut Transform)>
) {
    for (ball, mut ball_transform) in balls.iter_mut() {
        if ball.moving {
            ball_transform.translation.x += (ball.direction.sin())*BALL_SPEED;
            ball_transform.translation.y += (ball.direction.cos())*BALL_SPEED;
        }
    }
}

fn score_goal(
    mut balls: Query<(&mut Ball, &mut Transform)>
) {
    for (mut ball, mut ball_transform) in balls.iter_mut() {
        if ball_transform.translation.x <= -0.5*WINDOW_WIDTH {
            // Return to center, player 2 serve
            ball.direction = 0.5*std::f32::consts::PI;
            ball.moving = false;
            ball_transform.translation.x = 0.0;
            ball_transform.translation.y = 0.0;

            //TODO: Add scoring logic
        }
        
        if ball_transform.translation.x >= 0.5*WINDOW_WIDTH {
            // Return to center, player 1 serve
            ball.direction = -0.5*std::f32::consts::PI;
            ball.moving = false;
            ball_transform.translation.x = 0.0;
            ball_transform.translation.y = 0.0;

            //TODO: Add scoring logic
        }
    }
}