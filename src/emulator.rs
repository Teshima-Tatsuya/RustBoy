use crate::{
    gameboy::GameBoy,
    constant::*,
};
use bevy::{
    prelude::*,
    render::{
        render_resource::Extent3d,
        texture::{Image, ImageType,CompressedImageFormats},
    },
};

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
        .add_system(setup)
        .run();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    create_bg(&mut commands);
}

#[derive(Component)]
struct Background;

fn create_bg(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {

    let image = Image::from_buffer(
        vec![255,255,255],
        ImageType::Extension("bmp"),
        CompressedImageFormats::NONE,
        false,
    );

    let image_unwrap = image.unwrap();

    image_unwrap.resize(Extent3d {
        width: 10,
        height: 10,
        ..default()
    });

    let image_handle = images.add(image_unwrap);

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });


    commands
        .spawn_bundle(MaterualBundle {
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)), // サイズは適当
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background);
}