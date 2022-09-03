use crate::{constant::*, gameboy::GameBoy};
use bevy::{prelude::*, window::WindowResizeConstraints};

pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, app: &mut App) {
    }
}

fn emulator_system(
    // mut emulator: ResMut<Emulator>,
) {

}

pub struct Emulator {
    pub gb: GameBoy,
}

impl Emulator {
    pub fn new(gb: GameBoy) -> Self {
        Self { gb: gb }
    }

    pub fn init(&self) {
        let window_descriptor = WindowDescriptor {
            title: "rustboy".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            resize_constraints: WindowResizeConstraints {
                min_width: SCREEN_WIDTH as f32,
                min_height: SCREEN_HEIGHT as f32,
                ..default()
            },
            ..default()
        };
        App::new()
            .insert_resource(window_descriptor)
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_plugins(DefaultPlugins)
            .run();
    }
}
