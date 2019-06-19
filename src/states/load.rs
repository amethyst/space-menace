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
    resources::{BulletImpactResource, BulletResource, Map, Layer},
    SCALE,
    // states::RunState,
};

#[derive(Default)]
pub struct LoadState {
    progress_counter: ProgressCounter,
    map_handle: Option<Handle<Map>>,
    background_sprite_sheet_handle: Option<SpriteSheetHandle>,
    truss_sprite_sheet_handle: Option<SpriteSheetHandle>,
    platform_sprite_sheet_handle: Option<SpriteSheetHandle>,
    marine_prefab_handle: Option<Handle<Prefab<AnimationPrefabData>>>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        {
            let loader = world.read_resource::<Loader>();
            self.map_handle = Some(
                loader.load(
                    "tilemaps/map.json",
                    JsonFormat,
                    &mut self.progress_counter,
                    &world.read_resource::<AssetStorage<Map>>(),
                )
            );
        }

        self.background_sprite_sheet_handle = Some(self.get_sprite_sheet_handle(
            world,
            "textures/background.png",
            "prefabs/background.ron",
        ));

        self.truss_sprite_sheet_handle = Some(self.get_sprite_sheet_handle(
            world,
            "textures/truss.png",
            "prefabs/truss.ron",
        ));

        self.platform_sprite_sheet_handle = Some(self.get_sprite_sheet_handle(
            world,
            "textures/platform.png",
            "prefabs/platform.ron",
        ));

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
        if self.progress_counter.is_complete() {
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

            let background_layer = map.layers.iter()
                .find(|layer| layer.name == "background").unwrap();
            let truss_layer = map.layers.iter()
                .find(|layer| layer.name == "truss").unwrap();
            let platform_layer = map.layers.iter()
                .find(|layer| layer.name == "platform").unwrap();

            let background_sprite_sheet_handle = self.background_sprite_sheet_handle
                .take()
                .unwrap();
            let truss_sprite_sheet_handle = self.truss_sprite_sheet_handle
                .take()
                .unwrap();
            let platform_sprite_sheet_handle = self.platform_sprite_sheet_handle
                .take()
                .unwrap();

            self.load_non_collision_layer(
                data.world,
                background_layer,
                background_sprite_sheet_handle,
                -30.,
                screen_height
            );
            self.load_non_collision_layer(
                data.world,
                truss_layer,
                truss_sprite_sheet_handle,
                -20.,
                screen_height
            );
            self.load_non_collision_layer(
                data.world,
                platform_layer,
                platform_sprite_sheet_handle,
                -10.,
                screen_height
            );

            load_marine(data.world, self.marine_prefab_handle.take().unwrap());
            Trans::Switch(Box::new(LazyLoadState {
                ..LazyLoadState::default()
            }))
        } else {
            Trans::None
        }
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

            loader.load(
                texture_path,
                ImageFormat::default(),
                &mut self.progress_counter,
                &texture_storage,
            )
        };
        let loader = &world.read_resource::<Loader>();
        let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            ron_path,
            SpriteSheetFormat(texture_handle),
            &mut self.progress_counter,
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
            loader.load(ron_path, RonFormat, &mut self.progress_counter)
        })
    }

    fn load_non_collision_layer(
        &mut self,
        world: &mut World,
        layer: &Layer,
        sprite_sheet_handle: SpriteSheetHandle,
        z_transform: f32,
        screen_height: f32
    ) {
        for obj in layer.objects.iter() {
            let mut transform = Transform::default();

            transform.set_translation_xyz(
                (obj.x + obj.width / 2.) * SCALE,
                screen_height / 2. - (obj.y + obj.height / 2.) * SCALE,
                z_transform
            );

            transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));

            let sprite_index_prop = obj.properties.iter().find(
                |prop| prop.name == "spriteindex"
            );
            let mut sprite = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: 0,
            };

            match sprite_index_prop {
                Some(prop) => {
                    sprite = SpriteRender {
                        sprite_sheet: sprite_sheet_handle.clone(),
                        sprite_number: prop.value,
                    };
                },
                None => {},
            }

            world.create_entity()
                .with(transform)
                .with(sprite)
                .with(Transparent)
                .build();
        }
    }
}

#[derive(Default)]
pub struct LazyLoadState {
    progress_counter: ProgressCounter,
}

impl SimpleState for LazyLoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Init physics world
        let mut dispatcher = physics_dispatcher::<f32, Coordinates>();
        dispatcher.setup(&mut world.res);
        // let gravity = self.set_gravity(world, Vector3::new(0., -9.8, 0.));

        // let marine_sprite_sheet = self.get_sprite_sheet_handle(world, "textures/marine.png", "prefabs/marine.ron");
        // load sprite animations
        // self.marine_prefab_handle = Some(self.get_animation_prefab_handle(
        //     world,
        //     "prefabs/marine_new.ron"
        // ));
        // load_marine(world, marine_sprite_sheet, marine_animation_handle);
        // load_marine(world, marine_prefab_handle);

        // let mut bullet = world.write_resource::<BulletResource>();
        // bullet.sprite_sheet = Some(self.get_sprite_sheet_handle(
        //     world,
        //     "textures/bullet.png",
        //     "prefabs/bullet.ron",
        // ));
        // add_bullet_resource(world, self.bullet_spritesheet_handle.clone().unwrap());

        // let mut bullet_impact = world.write_resource::<BulletImpactResource>();
        // bullet_impact.sprite_sheet = Some(self.get_sprite_sheet_handle(
        //     world,
        //     "textures/bullet_impact.png",
        //     "prefabs/bullet_impact.ron",
        // ));
        // add_bullet_resource_impact(world, bullet_impact_spritesheet_handle);
    }

    // fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    //     if self.progress_counter.is_complete() {
    //         Trans::Switch(Box::new(RunState {
    //             // marine_prefab_handle: Some(self.marine_prefab_handle
    //             //     .take()
    //             //     .expect("Expected `marine_prefab_handle` to exist.")),
    //         }))
    //     } else {
    //         Trans::None
    //     }
    // }
}

impl LazyLoadState {
    // fn set_gravity(&mut self, world: &mut World, deceleration: Vector3<f32>) {
    //     let gravity = world.write_resource::<Gravity<f32>>();
    //     gravity.0 = deceleration;
    // }

    /// Returns a `SpriteSheetHandle` for the given texture and ron files.
    fn get_sprite_sheet_handle(
        &mut self,
        world: &World,
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
            loader.load(
                texture_path,
                ImageFormat::default(),
                // &mut self.progress_counter,
                (),
                &texture_storage,
            )
        };
        let loader = &world.read_resource::<Loader>();
        let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            ron_path,
            SpriteSheetFormat(texture_handle),
            // &mut self.progress_counter,
            (),
            &sprite_sheet_store,
        )
    }
}
