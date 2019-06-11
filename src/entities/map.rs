use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::{
        sprite::{SpriteSheetHandle, TextureCoordinates},
        Sprite,
        SpriteRender,
        SpriteSheet,
        Texture,
        transparent::Transparent,
    },
};

use tiled::{Map, Tileset};

use crate::SCALE;

pub fn load_map_layer(world: &mut World, map: &Map, tileset_id: u32, layer_index: usize, screen_width: f32, screen_height: f32, texture_handle: Handle<Texture>) {
    if let Some(map_tileset) = map.get_tileset_by_gid(tileset_id) {
        let tile_width = map_tileset.tile_width as i32;
        let tile_height = map_tileset.tile_height as i32;
        let tileset_sprites = get_tileset_sprites(map_tileset, tile_width, tile_height);
        let sprite_sheet_handle = get_tileset_sprite_sheet_handle(world, texture_handle, tileset_sprites);

        // Now that all the tile sprites/textures are loaded in
        // we can start drawing the tiles for our viewing pleasure
        let layer: &tiled::Layer = &map.layers[layer_index];
        let mut prev_layers_tile_cnt = 0;
        let z_transform = -1. * (map.layers.len() - layer_index) as f32;

        for i in 0..layer_index {
            prev_layers_tile_cnt += map.layers[i].tiles.len() as u32 * map.tilesets[i].images[0].width as u32 / map.tile_width;
        }

        for (y, row) in layer.tiles.iter().enumerate().clone() {
            for (x, &tile) in row.iter().enumerate() {

                // Do nothing with empty tiles
                if tile == 0 {
                    continue;
                }

                // Tile ids start from 1 but tileset sprites start from 0
                // Also, need to subtract the tilecount of previous layers
                // In this case, there is only one such layer, the "background" whose tilecount is 132
                let tile = tile - prev_layers_tile_cnt - 1;
                let tile_sprite = get_tile_sprite(tile, &sprite_sheet_handle);
                let tile_transform = get_tile_transform(x, y, tile_width, tile_height, screen_width, screen_height, z_transform);

                world
                    .create_entity()
                    .with(tile_transform)
                    .with(tile_sprite)
                    .with(Transparent)
                    .build();
            }

        }
    }
}

fn get_tileset_sprites(tileset: &Tileset, tile_width: i32, tile_height: i32) -> Vec<Sprite> {
    let tileset_width = &tileset.images[0].width;
    let tileset_height = &tileset.images[0].height;

    let tileset_sprite_columns = tileset_width / tile_width as i32;
    let tileset_sprite_offset_columns = 1.0 / tileset_sprite_columns as f32;

    let tileset_sprite_rows = tileset_height / tile_height as i32;
    let tileset_sprite_offset_rows = 1.0 / tileset_sprite_rows as f32;

    println!("tileset_sprite_rows = {}", tileset_sprite_rows);
    println!("tileset_sprite_columns = {}", tileset_sprite_columns);
    
    // A place to store the tile sprites in
    let mut tileset_sprites: Vec<Sprite> = Vec::new();

    // The x-axis needs to be reversed for TextureCoordinates
    for x in (0..tileset_sprite_rows).rev() {
        for y in 0..tileset_sprite_columns {
            // Coordinates of the 16x16 tile sprite inside the whole tileset image
            // Important: TextureCoordinates Y axis goes from BOTTOM (0.0) to TOP (1.0)
            let tex_coords = TextureCoordinates {
                left: y as f32 * tileset_sprite_offset_columns,
                right: (y + 1) as f32 * tileset_sprite_offset_columns,
                bottom: x as f32 * tileset_sprite_offset_rows * -1., // multiplying by -1 otherwise y-axis getting reversed
                top: (x + 1) as f32 * tileset_sprite_offset_rows * -1.,
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

fn get_tile_transform(row_index: usize, col_index: usize, tile_width: i32, tile_height: i32, screen_width: f32, screen_height: f32, z_transform: f32) -> Transform {
    // Where we should draw the tile?
    let mut tile_transform = Transform::default();
    let x_coord = SCALE as usize * row_index * tile_width as usize;
    // Bottom Left is 0,0 so we flip it to Top Left with the
    // ScreenDimensions.height since tiled coordinates start from top
    let y_coord = screen_height - (SCALE * col_index as f32 * tile_height as f32);
    // Offset the positions by half the tile size so they're nice and snuggly on the screen
    // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
    let offset_x = SCALE * (tile_width as f32) / 2.0;
    let offset_y = SCALE * -(tile_height as f32) / 2.0;

    tile_transform.set_translation_xyz(
        offset_x + x_coord as f32,
        offset_y + y_coord as f32,
        z_transform,
    );
    tile_transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));
    tile_transform
}
