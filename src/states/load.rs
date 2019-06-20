use amethyst::{
    assets::{AssetStorage, Handle, Loader, JsonFormat, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    core::Transform,
    ecs::prelude::World,
    prelude::{Builder, GameData, SimpleState, SimpleTrans, StateData, Trans},
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet,
        Texture,
        transparent::Transparent,
    },
    window::ScreenDimensions,
};

use specs_physics::{physics_dispatcher, Gravity, math::Vector3};

use crate::{
    components::{AnimationPrefabData, Coordinates},
    entities::{
        // Background,
        load_camera_subject,
        load_camera,
        // load_collider,
        // load_map_layer,
        load_marine,
        // load_background
    },
    resources::{AssetType, BulletImpactResource, BulletResource, Map, Layer, load_sprite_sheets, SpriteSheetList},
    SCALE,
    // states::RunState,
};

#[derive(Default)]
pub struct LoadState {
    progress_counter: Option<ProgressCounter>,
    map_handle: Option<Handle<Map>>,
    marine_prefab_handle: Option<Handle<Prefab<AnimationPrefabData>>>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.progress_counter = Some(load_sprite_sheets(world, vec![AssetType::Background, AssetType::Platform, AssetType::Truss]));

        {
            let loader = world.read_resource::<Loader>();
            self.map_handle = Some(
                loader.load(
                    "tilemaps/map.json",
                    JsonFormat,
                    self.progress_counter.as_mut().expect("map"),
                    &world.read_resource::<AssetStorage<Map>>(),
                )
            );
        }

        // let marine_sprite_sheet = self.get_sprite_sheet_handle(world, "textures/marine.png", "prefabs/marine.ron");
        self.marine_prefab_handle = Some(self.get_animation_prefab_handle(
            world,
            "prefabs/marine_new.ron",
        ));

        let camera_subject = load_camera_subject(world);
        load_camera(world, camera_subject);

        world.add_resource(BulletResource {
            sprite_sheet: self.get_sprite_sheet_handle(
                world,
                "textures/bullet.png",
                "prefabs/bullet.ron",
            ),
        });

        world.add_resource(BulletImpactResource {
            sprite_sheet: self.get_sprite_sheet_handle(
                world,
                "textures/bullet.png",
                "prefabs/bullet.ron",
            ),
        });
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data
        if let Some(ref progress_counter) = self.progress_counter {
            // Checks progress
            if progress_counter.is_complete() {
                // Get the map, which is loaded in the previous load state.
                let map: Map;
                {
                    let map_storage = &data.world.read_resource::<AssetStorage<Map>>();
                    let map_handle = &self.map_handle.clone().unwrap();
                    map = map_storage.get(map_handle).unwrap().clone();
                }

                let screen_height = {
                    let dim = data.world.read_resource::<ScreenDimensions>();
                    dim.height()
                };

                map.load_non_collision_layer(
                    data.world,
                    AssetType::Background,
                    -30.,
                    screen_height
                );
                map.load_non_collision_layer(
                    data.world,
                    AssetType::Truss,
                    -20.,
                    screen_height
                );
                map.load_non_collision_layer(
                    data.world,
                    AssetType::Platform,
                    -10.,
                    screen_height
                );

                load_marine(data.world, self.marine_prefab_handle.take().unwrap());

                self.progress_counter = None;
            }
        }
        Trans::None
    }
}

impl LoadState {
    /// Returns a `SpriteSheetHandle` for the given texture and ron files.
    fn get_sprite_sheet_handle(
        &mut self, world: &World,
        texture_path: &str,
        ron_path: &str
    ) -> SpriteSheetHandle {
        // Load the sprite sheet necessary to render the graphics.
        // The texture is the pixel data
        // `sprite_sheet` is the layout of the sprites on the image
        // `texture_handle` is a cloneable reference to the texture
        let texture_handle = {
            let loader = &world.read_resource::<Loader>();
            let texture_storage = &world.read_resource::<AssetStorage<Texture>>();

            println!("sprite sheet handle texture");
            loader.load(
                texture_path,
                ImageFormat::default(),
                self.progress_counter.as_mut().expect("texture"),
                &texture_storage,
            )
        };
        let loader = &world.read_resource::<Loader>();
        let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
        println!("sprite sheet handle sprite sheet");
        loader.load(
            ron_path,
            SpriteSheetFormat(texture_handle),
            self.progress_counter.as_mut().expect("sprite sheet"),
            &sprite_sheet_store,
        )
    }

    /// Loads a `Prefab` with type `AnimationPrefabData` from the given path.
    fn get_animation_prefab_handle(
        &mut self, world:
        &mut World,
        ron_path: &str
    ) -> Handle<Prefab<AnimationPrefabData>> {
        world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
            loader.load(ron_path, RonFormat, self.progress_counter.as_mut().expect("prefab"))
        })
    }
}
