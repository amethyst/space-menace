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

pub mod camera_subject_entity;
pub mod camera_entity;
pub mod marine_entity;
pub mod background_entity;
pub mod truss_entity;
pub mod platform_entity;

pub fn initialise_entities(world: &mut World) {
    let camera_subject = camera_subject_entity::get_entity(world);

    camera_entity::init(world, camera_subject);
    marine_entity::init(world);
    background_entity::init(world);
    truss_entity::init(world);
    platform_entity::init(world);
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