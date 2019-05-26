use amethyst::{
    assets::{ AssetStorage, Loader },
    core::{ Parent, Transform },
    ecs::{ Entity },
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, ScreenDimensions, Sprite, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, Transparent
    },
};

use crate::{
    BG_SCALE,
    BG_STRUCTURES_SCALE,
    BG_STRUCTURES_Z_TRANSFORM,
    BG_STRUCTURES_TILE_HEIGHT,
    BG_STRUCTURES_TILE_WIDTH,
    BG_TILE_HEIGHT,
    BG_TILE_WIDTH,
    BG_Z_TRANSFORM,
    MARINE_SCALE,
    PLATFORM_SCALE,
    PLATFORM_Z_TRANSFORM,
    components::{ Player, TwoDimObject, CameraSubject }
};

pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Player>();
        let sprite_sheet_handle = load_player_sprite_sheet(world);
        init_player(world, &sprite_sheet_handle);

        world.register::<CameraSubject>();
        let camera_subject = init_camera_subject(world);

        let bg_sprite_sheet_handle =
            load_sprite_sheet(world, "sprites/bg_tile.png", "prefabs/bg_tile.ron");
        init_bg_sprite(world, &bg_sprite_sheet_handle);

        let bg_sprite_sheet_handle =
            load_sprite_sheet(world, "sprites/bg_structures_tile.png", "prefabs/bg_structures_tile.ron");
        init_bg_structures_sprite(world, &bg_sprite_sheet_handle);

        let platform_tileset_sprite_sheet_handle =
            load_platform_tileset_sprite_sheet(world);
        init_platforms(world, &platform_tileset_sprite_sheet_handle);


        init_camera(world, camera_subject);
    }
}

fn init_camera_subject (world: &mut World) -> Entity {
    let mut transform = Transform::default();

    let mut two_dim_object = TwoDimObject::new(0., 0.);
    two_dim_object.set_position(384., 176.);
    two_dim_object.update_transform_position(&mut transform);

    world
        .create_entity()
        .with(transform)
        .with(CameraSubject::new(two_dim_object))
        .with(Transparent)
        .build()
}

fn init_camera(world: &mut World, camera_subject: Entity) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1.0);
    println!("width = {}, height = {}", width, height);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -1. * width / 2.,
            width / 2.,
            -1. * height / 2.,
            height / 2.,
            // 0.,
            // width,
            // 0.,
            // height,
        )))
        .with(Parent { entity: camera_subject })
        .with(transform)
        .build();
}

fn init_platforms(world: &mut World, sprite_sheet_handle: &SpriteSheetHandle) {
    for i in 0..5 {
        let mut transform = Transform::default();
        let sprite_number;
        let tile_w;
        let tile_h;
        let tile_left;
        transform.set_z(PLATFORM_Z_TRANSFORM);
        transform.set_scale(PLATFORM_SCALE, PLATFORM_SCALE, PLATFORM_SCALE);
        match i {
            0 => {
                sprite_number = 4;
                tile_w = 96.;
                tile_h = 40.;
                tile_left = 0.;
            },
            4 => {
                sprite_number = 2;
                tile_w = 96.;
                tile_h = 64.;
                tile_left = i as f32 * tile_w * PLATFORM_SCALE;
            },
            _ => {
                sprite_number = 4;
                tile_w = 96.;
                tile_h = 40.; // NOTE: 55 - 16
                tile_left = i as f32 * tile_w * PLATFORM_SCALE;
            }
        }
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: sprite_number,
        };

        let mut two_dim_object = TwoDimObject::new(tile_w * PLATFORM_SCALE, tile_h * PLATFORM_SCALE);
        two_dim_object.set_left(tile_left);
        two_dim_object.set_bottom(0.);
        two_dim_object.update_transform_position(&mut transform);

        world.create_entity()
            .with(transform)
            .with(two_dim_object)
            .with(sprite)
            .with(Transparent)
            .build();
    }
}

fn init_player(world: &mut World, sprite_sheet_handle: &SpriteSheetHandle) {
    let scale = MARINE_SCALE;

    let mut transform = Transform::default();
    transform.set_scale(scale, scale, scale);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 4, // paddle is the first sprite in the sprite_sheet
    };

    let mut two_dim_object = TwoDimObject::new(48. * MARINE_SCALE, 48. * MARINE_SCALE);
    two_dim_object.set_position(384., 176.);
    two_dim_object.update_transform_position(&mut transform);

    world
        .create_entity()
        .with(transform)
        .with(Player::new(two_dim_object))
        .with(sprite_render)
        .with(Transparent)
        .build();
}

fn init_bg_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) {
    for i in 0..4 {
        let mut transform = Transform::default();
        transform.set_xyz(0., 0., BG_Z_TRANSFORM);
        transform.set_scale(BG_SCALE, BG_SCALE, BG_SCALE);
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };

        let mut two_dim_object = TwoDimObject::new(BG_TILE_WIDTH * BG_SCALE, BG_TILE_HEIGHT * BG_SCALE);
        two_dim_object.set_left(i as f32 * BG_TILE_WIDTH * BG_SCALE);
        two_dim_object.set_bottom(0 as f32);
        two_dim_object.update_transform_position(&mut transform);

        world.create_entity()
            .with(transform)
            .with(sprite)
            .with(Transparent)
            .build();
    }
}

fn init_bg_structures_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) {
    for i in 0..2 {
        let mut transform = Transform::default();
        transform.set_xyz(0., 0., BG_STRUCTURES_Z_TRANSFORM);
        transform.set_scale(BG_STRUCTURES_SCALE, BG_STRUCTURES_SCALE, BG_STRUCTURES_SCALE);
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };

        let mut two_dim_object = TwoDimObject::new(BG_STRUCTURES_TILE_WIDTH * BG_STRUCTURES_SCALE, BG_STRUCTURES_TILE_HEIGHT * BG_STRUCTURES_SCALE);
        two_dim_object.set_left(i as f32 * BG_STRUCTURES_TILE_WIDTH * BG_SCALE);
        two_dim_object.set_bottom(0 as f32);
        two_dim_object.update_transform_position(&mut transform);

        world.create_entity()
            .with(transform)
            .with(sprite)
            .with(Transparent)
            .build();
    }
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

fn load_platform_tileset_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/platform_tileset.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();

    let sprite_count = 5; // total number of sprites
    let mut sprites = Vec::with_capacity(sprite_count);

    let image_w = 224;
    let image_h = 192;

    for i in 0..(sprite_count as u32) {
        let offset_x;
        let offset_y;
        let tile_width;
        let tile_height;

        match i {
            0 => {
                offset_x = 0;
                offset_y = 0;
                tile_width = 16;
                tile_height = 192;
            },
            1 => {
                offset_x = 32;
                offset_y = 25;
                tile_width = 64;
                tile_height = 71;
            },
            2 => {
                offset_x = 112;
                offset_y = 17;
                tile_width = 96;
                tile_height = 79;
            },
            3 => {
                offset_x = 32;
                offset_y = 108;
                tile_width = 64;
                tile_height = 68;
            },
            4 => {
                offset_x = 112;
                offset_y = 103;
                tile_width = 96;
                tile_height = 57;
            },
            _ => {
                offset_x = 0;
                offset_y = 0;
                tile_width = 0;
                tile_height = 0;
            }
        }
        let offsets = [0.; 2]; // Align the sprite with the middle of the entity.

        let sprite = Sprite::from_pixel_values(
            image_w, image_h, tile_width, tile_height, offset_x, offset_y, offsets,
        );
        sprites.push(sprite);
    }

    let sprite_sheet = SpriteSheet {
        texture: texture_handle,
        sprites,
    };

    loader.load_from_data(
        sprite_sheet,
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

fn load_player_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/marine_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();

    let sprite_count = 27; // total number of sprites
    let mut sprites = Vec::with_capacity(sprite_count);

    let image_w = 1384;
    let image_h = 48;

    for i in 0..(sprite_count as u32) {
        let offset_x;
        let player_width;
        if i < 4 {
            player_width = 80;
            offset_x = 80 * i;
        } else if i < 8 {
            player_width = 48;
            offset_x = 80 * 4 + 48 * (i - 4);
        } else if i < 14 {
            player_width = 36;
            offset_x = 80 * 4 + 48 * 4 + 36 * (i - 8);
        } else if i < 25 {
            player_width = 48;
            offset_x = 80 * 4 + 48 * 4 + 36 * 6 + 48 * (i - 14);
        } else {
            player_width = 64;
            offset_x = 80 * 4 + 48 * 4 + 36 * 6 + 48 * 11 + 64 * (i - 25);
        }
        let offset_y = 0;
        let offsets = [0.; 2]; // Align the sprite with the middle of the entity.

        let sprite = Sprite::from_pixel_values(
            image_w, image_h, player_width, 48, offset_x, offset_y, offsets,
        );
        sprites.push(sprite);
    }

    let sprite_sheet = SpriteSheet {
        texture: texture_handle,
        sprites,
    };

    loader.load_from_data(
        sprite_sheet,
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}