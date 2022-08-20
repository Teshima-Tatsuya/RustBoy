use crate::{
    gameboy::GameBoy,
    constant::*,
};
use bevy::prelude::*;

pub struct Emulator {
    pub gb: GameBoy,
}

impl Emulator {
    pub fn new(gb: GameBoy) -> Self {
        Self {
            gb: gb,
        }
    }

    pub fn init(&self) {
        App::new()
        .insert_resource(WindowDescriptor {
            title: "rustboy".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(175f32, 197f32, 160f32)))
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
    }
}