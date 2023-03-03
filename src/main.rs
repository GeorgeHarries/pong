use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


pub const ASPECT_RATIO: f32 = 16.0/9.0;
pub const WINDOW_HEIGHT: f32 =  720.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT*ASPECT_RATIO;

pub const RACKET_HEIGHT: f32 = 25.0;
pub const RACKET_WIDTH: f32 = 100.0;
pub const RACKET_EDGE_OFFSET: f32 = 50.0;

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
        .add_plugin(WorldInspectorPlugin)
        .register_type::<Racket>()
        .register_type::<Player>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_rackets)
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
        transform: Transform::from_xyz(-WINDOW_WIDTH/2.0+RACKET_EDGE_OFFSET, 0.0, 0.0),
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
        transform: Transform::from_xyz(WINDOW_WIDTH/2.0-RACKET_EDGE_OFFSET, 0.0, 0.0),
        ..default()
    })
    .insert(Name::new("Player 2 Racket"))
    .insert(Racket)
    .insert(Player {player_number: 2});
}

