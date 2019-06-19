use amethyst::{
    assets::{AssetStorage, Handle, Loader, JsonFormat, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    core::Transform,
    ecs::prelude::World,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet,
        Texture,
        transparent::Transparent,
    },
};

// /// Returns a `SpriteSheetHandle` for the given texture and ron files.
// pub fn get_sprite_sheet_handle(
//     world: &World,
//     texture_path: &str,
//     ron_path: &str,
//     progress_counter: &mut Option<ProgressCounter>
// ) -> SpriteSheetHandle {
//     // Load the sprite sheet necessary to render the graphics.
//     // The texture is the pixel data
//     // `sprite_sheet` is the layout of the sprites on the image
//     // `texture_handle` is a cloneable reference to the texture
//     let texture_handle = {
//         let loader = &world.read_resource::<Loader>();
//         let texture_storage = &world.read_resource::<AssetStorage<Texture>>();

//         loader.load(
//             texture_path,
//             ImageFormat::default(),
//             progress_counter.as_mut().unwrap(),
//             &texture_storage,
//         )
//     };
//     let loader = &world.read_resource::<Loader>();
//     let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
//     loader.load(
//         ron_path,
//         SpriteSheetFormat(texture_handle),
//         progress_counter.as_mut().unwrap(),
//         &sprite_sheet_store,
//     )
// }
