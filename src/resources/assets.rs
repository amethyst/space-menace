use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, JsonFormat, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    core::Transform,
    ecs::prelude::World,
    prelude::Builder,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet,
        Texture,
        transparent::Transparent,
    },
};

use specs_physics::{math::Vector3};

use crate::{
    resources::{Map, Layer},
    SCALE,
};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType {
    Background,
    Platform,
    Truss,
}

#[derive(Default)]
pub struct SpriteSheetList {
    sprite_sheets: HashMap<AssetType, SpriteSheetHandle>,
}

impl SpriteSheetList {
    pub fn insert(
        &mut self,
        asset_type: AssetType,
        sprite_sheet_handle: SpriteSheetHandle,
    ) {
        self.sprite_sheets.insert(asset_type, sprite_sheet_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&asset_type)
    }
}

pub fn load_sprite_sheets(world: &mut World, asset_type_list: Vec<AssetType>) -> ProgressCounter {
    let mut sprite_sheet_list = SpriteSheetList::default();
    let mut progress_counter = ProgressCounter::new();

    for &asset_type in asset_type_list.iter() {
        let (texture_path, ron_path) = match asset_type {
            AssetType::Background => {
                ("textures/background.png", "prefabs/background.ron")
            },
            AssetType::Platform => {
                ("textures/platform.png", "prefabs/platform.ron")
            },
            AssetType::Truss => {
                ("textures/truss.png", "prefabs/truss.ron")
            },
        };
        let sprite_sheet_handle = get_sprite_sheet_handle(world, texture_path, ron_path, &mut progress_counter);
        sprite_sheet_list.insert(asset_type, sprite_sheet_handle);
    }
    world.add_resource(sprite_sheet_list);
    progress_counter
}

/// Returns a `SpriteSheetHandle` for the given texture and ron files.
pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: &str,
    ron_path: &str,
    progress_counter: &mut ProgressCounter
) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            texture_path,
            ImageFormat::default(),
            // progress_counter,
            (),
            &texture_storage,
        )
    };
    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress_counter,
        &sprite_sheet_store,
    )
}
