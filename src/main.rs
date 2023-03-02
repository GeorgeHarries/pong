use bevy::prelude::*;

pub const WINDOW_WIDTH: f32  = 1280.0;
pub const WINDOW_HEIGHT: f32 =  720.0;

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
        .run();
}