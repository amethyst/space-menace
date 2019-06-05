use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    ecs::prelude::World,
    prelude::Builder,
    renderer::{
        PngFormat,
        ScreenDimensions,
        Sprite,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureCoordinates,
        TextureMetadata,
    },
};

use tiled::{Map, parse, Tileset};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::{
    SCALE, PLATFORM_Z_TRANSFORM,
    components::TwoDimObject,
};

pub mod camera_subject;
pub mod camera;
pub mod marine;
pub mod background;
pub mod truss;
pub mod platform;

mod bullet;

pub use self::bullet::spawn_bullet;
pub use self::bullet::show_bullet_impact;

pub fn initialise_entities(world: &mut World) {
    let camera_subject = camera_subject::get_entity(world);

    bullet::init_bullet_impact(world);
    bullet::init_bullet(world);
    camera::init(world, camera_subject);
    marine::init(world);

    // Get the game window screen height
    let screen_height = {
        let dim = world.read_resource::<ScreenDimensions>();
        dim.height()
    };

    let map = get_parsed_map_data("assets/tilemaps/map.tmx");
    // background::init(world);
    load_bg(world, &map, 1, screen_height);
    // create_entities_for_tiles(world, &map, 1, screen_height);
    // truss::init(world);
    load_truss(world, &map, 2, screen_height);
    // platform::init(world);
    load_platform(world, &map, 3, screen_height);

    load_collision_layer(world, &map, screen_height);
}

// fn create_entities_for_tiles(world: &mut World, map: &Map, tileset_id: u32, screen_height: f32) {
//     let texture_handle = get_texture_handle(world, "sprites/truss.png");

//     if let Some(map_tileset) = map.get_tileset_by_gid(tileset_id) {
//         let tile_width = map_tileset.tile_width as i32;
//         let tile_height = map_tileset.tile_height as i32;
//         let tileset_sprites = get_tileset_sprites(map_tileset, tile_width, tile_height);
//         let sprite_sheet_handle = get_tileset_sprite_sheet_handle(world, texture_handle, tileset_sprites);

//         // Now that all the tile sprites/textures are loaded in
//         // we can start drawing the tiles for our viewing pleasure
//         let layer: &tiled::Layer = &map.layers[1];

//         for (y, row) in layer.tiles.iter().enumerate().clone() {
//             for (x, &tile) in row.iter().enumerate() {
//                 // Do nothing with empty tiles
//                 if tile == 0 {
//                     continue;
//                 }

//                 // Tile ids start from 1 but tileset sprites start from 0
//                 // Also, need to subtract the tilecount of previous layers
//                 // In this case, there is only one such layer, the "background" whose tilecount is 132
//                 // let tile = tile - 1;
//                 let tile = tile - 132 - 1;
//                 let tile_sprite = get_tile_sprite(tile, &sprite_sheet_handle);
//                 let tile_transform = get_tile_transform(x, y, tile_width, tile_height, screen_height, -20.);

//                 world
//                     .create_entity()
//                     .with(tile_transform)
//                     .with(tile_sprite)
//                     .build();
//             }

//         }
//     }
// }

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

fn load_bg(world: &mut World, map: &Map, tileset_id: u32, screen_height: f32) {
    let texture_handle = get_texture_handle(world, "sprites/bg.png");

    if let Some(map_tileset) = map.get_tileset_by_gid(tileset_id) {
        let tile_width = map_tileset.tile_width as i32;
        let tile_height = map_tileset.tile_height as i32;
        let tileset_sprites = get_tileset_sprites(map_tileset, tile_width, tile_height);
        let sprite_sheet_handle = get_tileset_sprite_sheet_handle(world, texture_handle, tileset_sprites);

        // Now that all the tile sprites/textures are loaded in
        // we can start drawing the tiles for our viewing pleasure
        let layer: &tiled::Layer = &map.layers[0];

        // Loop the row first and then the individual tiles on that row
        // and then switch to the next row
        // y = row number
        // x = column number
        for (y, row) in layer.tiles.iter().enumerate().clone() {
            for (x, &tile) in row.iter().enumerate() {
                // Do nothing with empty tiles
                if tile == 0 {
                    continue;
                }

                // Tile ids start from 1 but tileset sprites start from 0
                let tile = tile - 1;
                let tile_sprite = get_tile_sprite(tile, &sprite_sheet_handle);
                let tile_transform = get_tile_transform(x, y, tile_width, tile_height, screen_height, -30.);

                // Create the tile entity
                world
                    .create_entity()
                    .with(tile_transform)
                    .with(tile_sprite)
                    .build();
            }

        }
    }
}

fn load_truss(world: &mut World, map: &Map, tileset_id: u32, screen_height: f32) {
    let texture_handle = get_texture_handle(world, "sprites/truss.png");

    if let Some(map_tileset) = map.get_tileset_by_gid(tileset_id) {
        let tile_width = map_tileset.tile_width as i32;
        let tile_height = map_tileset.tile_height as i32;
        let tileset_sprites = get_tileset_sprites(map_tileset, tile_width, tile_height);
        let sprite_sheet_handle = get_tileset_sprite_sheet_handle(world, texture_handle, tileset_sprites);

        // Now that all the tile sprites/textures are loaded in
        // we can start drawing the tiles for our viewing pleasure
        let layer: &tiled::Layer = &map.layers[1];

        for (y, row) in layer.tiles.iter().enumerate().clone() {
            for (x, &tile) in row.iter().enumerate() {
                // Do nothing with empty tiles
                if tile == 0 {
                    continue;
                }

                // Tile ids start from 1 but tileset sprites start from 0
                // Also, need to subtract the tilecount of previous layers
                // In this case, there is only one such layer, the "background" whose tilecount is 132
                // let tile = tile - 1;
                let tile = tile - 132 - 1;
                let tile_sprite = get_tile_sprite(tile, &sprite_sheet_handle);
                let tile_transform = get_tile_transform(x, y, tile_width, tile_height, screen_height, -20.);

                world
                    .create_entity()
                    .with(tile_transform)
                    .with(tile_sprite)
                    .build();
            }

        }
    }
}

fn load_platform(world: &mut World, map: &Map, tileset_id: u32, screen_height: f32) {
    let texture_handle = get_texture_handle(world, "sprites/platform.png");

    if let Some(map_tileset) = map.get_tileset_by_gid(tileset_id) {
        let tile_width = map_tileset.tile_width as i32;
        let tile_height = map_tileset.tile_height as i32;
        let tileset_sprites = get_tileset_sprites(map_tileset, tile_width, tile_height);
        let sprite_sheet_handle = get_tileset_sprite_sheet_handle(world, texture_handle, tileset_sprites);

        // Now that all the tile sprites/textures are loaded in
        // we can start drawing the tiles for our viewing pleasure
        let layer: &tiled::Layer = &map.layers[2];

        for (y, row) in layer.tiles.iter().enumerate().clone() {
            for (x, &tile) in row.iter().enumerate() {
                // Do nothing with empty tiles
                if tile == 0 {
                    continue;
                }

                // Tile ids start from 1 but tileset sprites start from 0
                // Also, need to subtract the tilecount of previous layers
                // In this case, there are 2 such layers, "background" (tilecount = 132) and "truss" (tilecount = 286)
                let tile = tile - 418 - 1;
                let tile_sprite = get_tile_sprite(tile, &sprite_sheet_handle);
                let tile_transform = get_tile_transform(x, y, tile_width, tile_height, screen_height, PLATFORM_Z_TRANSFORM);

                world
                    .create_entity()
                    .with(tile_transform)
                    .with(tile_sprite)
                    // .with(two_dim_object)
                    .build();
            }

        }
    }
}

fn load_collision_layer(world: &mut World, map: &Map, screen_height: f32) {
    let collision_objects = &map.object_groups[0].objects;

    for obj in collision_objects.iter() {
        let mut transform = Transform::default();
        transform.set_z(PLATFORM_Z_TRANSFORM);

        let mut two_dim_object = TwoDimObject::new(16. * SCALE, 16. * SCALE);
        two_dim_object.set_left(obj.x * SCALE);
        two_dim_object.set_top(screen_height - (obj.y * SCALE));
        two_dim_object.update_transform_position(&mut transform);

        world.create_entity()
            .with(transform)
            .with(two_dim_object)
            .build();
    }
}

fn get_texture_handle(world: &World, file_path: &str) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        file_path,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage
    )
}

fn get_parsed_map_data(map_path: &str) -> Map {
    let file = File::open(&Path::new(map_path)).unwrap();
    let reader = BufReader::new(file);
    parse(reader).unwrap()
}

fn get_tileset_sprites(tileset: &Tileset, tile_width: i32, tile_height: i32) -> Vec<Sprite> {
    let tileset_width = &tileset.images[0].width;
    let tileset_height = &tileset.images[0].height;

    let tileset_sprite_columns = tileset_width / tile_width as i32;
    let tileset_sprite_offset_colums = 1.0 / tileset_sprite_columns as f32;

    let tileset_sprite_rows = tileset_height / tile_height as i32;
    let tileset_sprite_offset_rows = 1.0 / tileset_sprite_rows as f32;
    
    // A place to store the tile sprites in
    let mut tileset_sprites: Vec<Sprite> = Vec::new();

    // The x-axis needs to be reversed for TextureCoordinates
    for x in (0..tileset_sprite_rows).rev() {
        for y in 0..tileset_sprite_columns {
            
            // Coordinates of the 64x64 tile sprite inside the whole
            // tileset image, `terrainTiles_default.png` in this case
            // Important: TextureCoordinates Y axis goes from BOTTOM (0.0) to TOP (1.0)
            let tex_coords = TextureCoordinates {
                left: y as f32 * tileset_sprite_offset_colums,
                right: (y + 1) as f32 * tileset_sprite_offset_colums,
                bottom: x as f32 * tileset_sprite_offset_rows,
                top: (x + 1) as f32 * tileset_sprite_offset_rows
            };

            let sprite = Sprite {
                width: tile_width as f32,
                height: tile_height as f32,
                offsets: [0.0, 0.0],
                tex_coords
            };

            tileset_sprites.push(sprite);
        }
    }

    tileset_sprites
}

fn get_tileset_sprite_sheet_handle(world: &World, texture_handle: Handle<Texture>, sprites: Vec<Sprite>) -> SpriteSheetHandle {
    // A sheet of sprites.. so all the tile sprites
    let sprite_sheet = SpriteSheet {
        texture: texture_handle,
        sprites: sprites
    };

    // Insert the sprite sheet, which consists of all the tile sprites,
    // into world resources for later use
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
}

fn get_tile_sprite(tile_id: u32, sprite_sheet_handle: &SpriteSheetHandle) -> SpriteRender {
    // Sprite for the tile
    SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: tile_id as usize,
    }
}

fn get_tile_transform(row_index: usize, col_index: usize, tile_width: i32, tile_height: i32, screen_height: f32, z_transform: f32) -> Transform {
    // Where we should draw the tile?
    let mut tile_transform = Transform::default();
    let x_coord = 2 * row_index * tile_width as usize;
    // Bottom Left is 0,0 so we flip it to Top Left with the
    // ScreenDimensions.height since tiled coordinates start from top
    let y_coord = (screen_height) - (2. * col_index as f32 * tile_height as f32);
    // Offset the positions by half the tile size so they're nice and snuggly on the screen
    // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
    let offset_x = 2. * tile_width as f32 / 2.0;
    let offset_y = 2. * -tile_height as f32 / 2.0;

    tile_transform.set_xyz(
        offset_x + x_coord as f32,
        offset_y + y_coord as f32,
        z_transform
    );
    tile_transform.set_scale(2., 2., 2.);
    tile_transform
}
