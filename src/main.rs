use bevy::prelude::*;

pub const WINDOW_HEIGHT: f32 =  720.0;
pub const RESOLUTION: f32 = 16.0/9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(
            WindowPlugin { 
                window: WindowDescriptor{
                    width: WINDOW_HEIGHT*RESOLUTION,
                    height: WINDOW_HEIGHT,
                    title: "Bevy Pong Implementation".to_string(),
                    resizable: false,
                    ..Default::default()
                },
                ..default()
            }
        ))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_test_text)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_test_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle { 
        font: asset_server.load("fonts/arial.ttf"), 
        font_size: 60.0, 
        color: Color::WHITE
    };
    commands.spawn(Text2dBundle {
        text: Text::from_section("This is a test", text_style.clone())
            .with_alignment(TextAlignment::CENTER),
        ..default()
    });
}