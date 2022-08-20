use crate::{constant::*, gameboy::GameBoy};
use bevy::{prelude::*, window::WindowResizeConstraints};
use bevy_pixels::prelude::*;

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
            .insert_resource(PixelsOptions {
                width: SCREEN_WIDTH as u32,
                height: SCREEN_HEIGHT as u32,
            })
            .add_plugins(DefaultPlugins)
            .add_plugin(PixelsPlugin)
            .add_system(bevy::input::system::exit_on_esc_system)
            .add_system(main_system)
            .run();
    }
}

fn main_system(mut pixels_resource: ResMut<PixelsResource>) {
    let frame: &mut [u8] = pixels_resource.pixels.get_frame();
}
