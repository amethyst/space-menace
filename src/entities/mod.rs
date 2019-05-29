use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::World,
    renderer::{
        PngFormat,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
    },
};

pub mod camera_subject;
pub mod camera;
pub mod marine;
pub mod background;
pub mod truss;
pub mod platform;

mod bullet;

pub use self::bullet::spawn_bullet;

pub fn initialise_entities(world: &mut World) {
    let camera_subject = camera_subject::get_entity(world);

    bullet::init(world);
    camera::init(world, camera_subject);
    marine::init(world);
    background::init(world);
    truss::init(world);
    platform::init(world);
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            png_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}