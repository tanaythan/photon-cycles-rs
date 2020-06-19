mod bike;
mod resources;
mod systems;
mod wall;

use bike::Bike;
use resources::init_sprite_resource;
use systems::{BikeSystem, CollisionSystem, WallSystem};
use wall::Wall;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::{Transform, TransformBundle},
    ecs::World,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        Camera, ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    utils::application_root_dir,
};

const ARENA_WIDTH: f32 = 800f32;
const ARENA_HEIGHT: f32 = 600f32;
const OFFSET_Y: f32 = 50f32;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Bike>();
        world.register::<Wall>();
        let sprite_sheet_handle = load_sprite_sheet(world);

        init_camera(world);
        init_bike(world, sprite_sheet_handle.clone());
        init_sprite_resource(world, sprite_sheet_handle);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn init_bike(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut blue_transform = Transform::default();
    let mut red_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    blue_transform.set_translation_xyz(ARENA_WIDTH * 0.5, y + OFFSET_Y, 0.0);
    red_transform.set_translation_xyz(ARENA_WIDTH * 0.5, y - OFFSET_Y, 0.0);
    red_transform.set_rotation_z_axis(std::f32::consts::PI);

    let sprite_render_blue = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let sprite_render_red = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    let blue_ent = world
        .create_entity()
        .with(Bike::new(bike::Color::Blue))
        .with(blue_transform)
        .with(sprite_render_blue)
        .build();
    world.insert(blue_ent);

    let red_ent = world
        .create_entity()
        .with(Bike::new(bike::Color::Red))
        .with(red_transform)
        .with(sprite_render_red)
        .build();
    world.insert(red_ent);
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/bike_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/bike_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(BikeSystem, "bike_system", &["input_system"])
        .with(WallSystem::default(), "wall_system", &["bike_system"])
        .with(
            CollisionSystem,
            "collision_system",
            &["bike_system", "wall_system"],
        );

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
