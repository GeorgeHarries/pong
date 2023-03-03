use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


pub const ASPECT_RATIO: f32 = 16.0/9.0;
pub const WINDOW_HEIGHT: f32 =  720.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT*ASPECT_RATIO;

pub const RACKET_HEIGHT: f32 = 25.0;
pub const RACKET_WIDTH: f32 = 100.0;
pub const RACKET_EDGE_OFFSET: f32 = 50.0;

pub const RACKET_SPEED: f32 = 10.0;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Racket;

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
        .register_type::<Player>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_rackets)
        .add_system(control_rackets)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(Name::new("Camera"));
}

fn spawn_rackets(
    mut commands: Commands,
) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(RACKET_HEIGHT, RACKET_WIDTH)),
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
            custom_size: Some(Vec2::new(RACKET_HEIGHT, RACKET_WIDTH)),
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
            if player.player_number == 1 && transform.translation.y < 0.5*WINDOW_HEIGHT-RACKET_HEIGHT {
                transform.translation.y += RACKET_SPEED;
            }
        }
    }
    
    if keyboard.pressed(KeyCode::S) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 1 && transform.translation.y > -0.5*WINDOW_HEIGHT+RACKET_HEIGHT {
                transform.translation.y -= RACKET_SPEED;
            }
        }
    }
    
    if keyboard.pressed(KeyCode::Up) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 2 && transform.translation.y < 0.5*WINDOW_HEIGHT-RACKET_HEIGHT {
                transform.translation.y += RACKET_SPEED;
            }
        }
    }
    
    if keyboard.pressed(KeyCode::Down) {
        for (player, mut transform) in rackets.iter_mut() {
            if player.player_number == 2 && transform.translation.y > -0.5*WINDOW_HEIGHT+RACKET_HEIGHT {
                transform.translation.y -= RACKET_SPEED;
            }
        }
    }
}