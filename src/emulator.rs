use crate::{constant::*, gameboy::GameBoy};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::WindowResizeConstraints,
};
use bevy_tiled_camera::TiledCameraPlugin;
pub struct EmulatorPlugin;

impl Plugin for EmulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_emulator_system)
            .add_system(emulator_system);
    }
}

fn setup_emulator_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Please input rom file path as args 1");
        return;
    }

    let bytes = std::fs::read(&args[1]).unwrap();
    
    let gb = GameBoy::new(&bytes);
    let emulator = Emulator::new(gb);
    commands.insert_resource(emulator);

    let img = Image::new(
        Extent3d {
            width: SCREEN_WIDTH as u32,
            height: SCREEN_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![0; (SCREEN_WIDTH as u32 * SCREEN_HEIGHT as u32 * 4) as usize],
        TextureFormat::Rgba8UnormSrgb,
    );

    let texture = images.add(img);

    commands
        .spawn_bundle(SpriteBundle {
            texture: texture.clone(),
            ..Default::default()
        })
        .insert(ScreenSprite);

    commands.insert_resource(GameScreen(texture));
}

#[derive(Component)]
pub struct ScreenSprite;

pub struct GameScreen(pub Handle<Image>);

fn emulator_system(
    screen: Res<GameScreen>,
    mut emulator: ResMut<Emulator>,
    mut images: ResMut<Assets<Image>>,
) {
    emulator.gb.exec_frame();
    emulator.frame = emulator.frame.wrapping_add(1);

    let image_data = emulator.gb.display();

    let image = images.get_mut(&screen.0).unwrap();

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let ix = y as usize * SCREEN_WIDTH as usize + x as usize;
            let pixel = &mut image.data[ix * 4..ix * 4 + 4];
            let image_pixel: &[u8] = &image_data.get_pixel(x as u32, y as u32).0;
            pixel[0] = image_pixel[0];
            pixel[1] = image_pixel[1];
            pixel[2] = image_pixel[2];
            pixel[3] = image_pixel[3];
        }
    }
}

pub struct Emulator {
    pub gb: GameBoy,
    pub frame: u32,
}

impl Emulator {
    pub fn new(gb: GameBoy) -> Self {
        Self { gb: gb, frame: 0 }
    }

    pub fn run() {
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
            .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
            .add_plugins(DefaultPlugins)
            .add_plugin(TiledCameraPlugin)
            .add_plugin(EmulatorPlugin)
            .add_plugin(JoypadPlugin)
            .add_startup_system(setup)
            .run();
    }
}
fn setup(
    mut commands: Commands,
) {
    use bevy_tiled_camera::*;
    commands.spawn_bundle(TiledCameraBundle::pixel_cam([SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32]).with_pixels_per_tile([1, 1]));
}

pub struct JoypadPlugin;

impl Plugin for JoypadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(joypad_system);
    }
}

fn joypad_system(
    emulator: Res<Emulator>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::A) {
        println!("key A pressed");
        emulator.gb.joypad.lock().unwrap().press(crate::joypad::BUTTON_A);
    } else if keys.just_released(KeyCode::A) {
        emulator.gb.joypad.lock().unwrap().release(crate::joypad::BUTTON_A);
    } 
    if keys.just_pressed(KeyCode::B) {
        println!("key B pressed");
        emulator.gb.joypad.lock().unwrap().press(crate::joypad::BUTTON_B);
    } else if keys.just_released(KeyCode::B) {
        emulator.gb.joypad.lock().unwrap().release(crate::joypad::BUTTON_B);
    } 
    if keys.just_pressed(KeyCode::Up) {
        println!("key Up pressed");
        emulator.gb.joypad.lock().unwrap().press(crate::joypad::BUTTON_UP);
    } else if keys.just_released(KeyCode::Up) {
        emulator.gb.joypad.lock().unwrap().release(crate::joypad::BUTTON_UP);
    } 
    if keys.just_pressed(KeyCode::Left) {
        println!("key Left pressed");
        emulator.gb.joypad.lock().unwrap().press(crate::joypad::BUTTON_LEFT);
    } else if keys.just_released(KeyCode::Left) {
        emulator.gb.joypad.lock().unwrap().release(crate::joypad::BUTTON_LEFT);
    } 
    if keys.just_pressed(KeyCode::Right) {
        println!("key Right pressed");
        emulator.gb.joypad.lock().unwrap().press(crate::joypad::BUTTON_RIGHT);
    } else if keys.just_released(KeyCode::Right) {
        emulator.gb.joypad.lock().unwrap().release(crate::joypad::BUTTON_RIGHT);
    } 
    if keys.just_pressed(KeyCode::Down) {
        println!("key Down pressed");
        emulator.gb.joypad.lock().unwrap().press(crate::joypad::BUTTON_DOWN);
    } else if keys.just_released(KeyCode::Down) {
        emulator.gb.joypad.lock().unwrap().release(crate::joypad::BUTTON_DOWN);
    } 
    if keys.just_pressed(KeyCode::Return) {
        println!("key START pressed");
        emulator.gb.joypad.lock().unwrap().press(crate::joypad::BUTTON_START);
    } else if keys.just_released(KeyCode::Return) {
        emulator.gb.joypad.lock().unwrap().release(crate::joypad::BUTTON_START);
    } 
    if keys.just_pressed(KeyCode::Space) {
        println!("key SELECT pressed");
        emulator.gb.joypad.lock().unwrap().press(crate::joypad::BUTTON_SELECT);
    } else if keys.just_released(KeyCode::Space) {
        emulator.gb.joypad.lock().unwrap().release(crate::joypad::BUTTON_SELECT);
    } 
}
