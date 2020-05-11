use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::World,
    prelude::WorldExt,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
};

use crate::components::AnimationPrefabData;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType {
    Background,
    Bullet,
    BulletImpact,
    Marine,
    Pincer,
    Flier,
    Platform,
    SmallExplosion,
    Truss,
}

#[derive(Default)]
pub struct SpriteSheetList {
    sprite_sheets: HashMap<AssetType, SpriteSheetHandle>,
}

impl SpriteSheetList {
    pub fn insert(&mut self, asset_type: AssetType, sprite_sheet_handle: SpriteSheetHandle) {
        self.sprite_sheets.insert(asset_type, sprite_sheet_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&asset_type)
    }
}

#[derive(Default)]
pub struct PrefabList {
    prefabs: HashMap<AssetType, Handle<Prefab<AnimationPrefabData>>>,
}

impl PrefabList {
    pub fn insert(
        &mut self,
        asset_type: AssetType,
        prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    ) {
        self.prefabs.insert(asset_type, prefab_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&Handle<Prefab<AnimationPrefabData>>> {
        self.prefabs.get(&asset_type)
    }
}

/// Loads `SpriteSheetHandle`s for all the assets in the `AssetType` list into the `world`
pub fn load_assets(world: &mut World, asset_type_list: Vec<AssetType>) -> ProgressCounter {
    let mut sprite_sheet_list = SpriteSheetList::default();
    let mut prefab_list = PrefabList::default();
    let mut progress_counter = ProgressCounter::new();

    for &asset_type in asset_type_list.iter() {
        let (texture_path, ron_path) = match asset_type {
            AssetType::Background => ("textures/background.png", "prefabs/background.ron"),
            AssetType::Bullet => ("textures/bullet.png", "prefabs/bullet.ron"),
            AssetType::BulletImpact => ("", "prefabs/bullet_impact.ron"),
            AssetType::Marine => ("", "prefabs/marine.ron"),
            AssetType::Pincer => ("", "prefabs/pincer.ron"),
            AssetType::Flier => ("", "prefabs/flier.ron"),
            AssetType::Platform => ("textures/platform.png", "prefabs/platform.ron"),
            AssetType::SmallExplosion => (
                "textures/small_explosion.png",
                "prefabs/small_explosion.ron",
            ),
            AssetType::Truss => ("textures/truss.png", "prefabs/truss.ron"),
        };

        match asset_type {
            AssetType::Background | AssetType::Bullet | AssetType::Platform | AssetType::Truss => {
                let sprite_sheet_handle =
                    get_sprite_sheet_handle(world, texture_path, ron_path, &mut progress_counter);
                sprite_sheet_list.insert(asset_type, sprite_sheet_handle);
            }
            AssetType::BulletImpact
            | AssetType::Marine
            | AssetType::Pincer
            | AssetType::Flier
            | AssetType::SmallExplosion => {
                let prefab_handle =
                    get_animation_prefab_handle(world, ron_path, &mut progress_counter);
                prefab_list.insert(asset_type, prefab_handle);
            }
        };
    }
    world.insert(sprite_sheet_list);
    world.insert(prefab_list);
    progress_counter
}

/// Returns a `SpriteSheetHandle` for the given texture and ron files.
pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: &str,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(texture_path, ImageFormat::default(), (), &texture_storage)
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

/// Loads a `Prefab` with type `AnimationPrefabData` from the given path.
fn get_animation_prefab_handle(
    world: &mut World,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<AnimationPrefabData>> {
    world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load(ron_path, RonFormat, progress_counter)
    })
}
