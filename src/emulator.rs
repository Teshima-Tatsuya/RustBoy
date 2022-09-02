use crate::{constant::*, gameboy::GameBoy};
use bevy::{prelude::*, window::WindowResizeConstraints};

pub struct Emulator {
    pub gb: GameBoy,
}

impl Emulator {
    pub fn new(gb: GameBoy) -> Self {
        Self { gb: gb }
    }

    pub fn init(&self) {
        App::new()
            .insert_resource(WindowDescriptor {
                title: "rustboy".to_string(),
                width: SCREEN_WIDTH as f32,
                height: SCREEN_HEIGHT as f32,
                resize_constraints: WindowResizeConstraints {
                    min_width: SCREEN_WIDTH as f32,
                    min_height: SCREEN_HEIGHT as f32,
                    ..default()
                },
                ..default()
            })
            .add_plugins(DefaultPlugins)
            .add_system(bevy::input::system::exit_on_esc_system)
            .run();
    }
}