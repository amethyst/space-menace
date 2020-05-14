use amethyst::{
    assets::{AssetStorage, Handle, JsonFormat, Loader, ProgressCounter},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
    ui::UiCreator,
};

use crate::{
    entities::{load_camera, load_camera_subject, load_flier, load_marine, load_pincer},
    resources::{load_assets, AssetType, Context, Map, PrefabList},
};

#[derive(Default)]
pub struct LoadState {
    progress_counter: Option<ProgressCounter>,
    map_handle: Option<Handle<Map>>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.insert(Context::new());

        self.progress_counter = Some(load_assets(
            world,
            vec![
                AssetType::Background,
                AssetType::Bullet,
                AssetType::BulletImpact,
                AssetType::Marine,
                AssetType::Pincer,
                AssetType::Flier,
                AssetType::Platform,
                AssetType::SmallExplosion,
                AssetType::Truss,
            ],
        ));
        let mut progress = ProgressCounter::default();
        world.exec(|mut creator: UiCreator<'_>| creator.create("ui/fps.ron", &mut progress));
        world.exec(|mut creator: UiCreator<'_>| creator.create("ui/player.ron", &mut progress));
        self.map_handle = {
            let loader = world.read_resource::<Loader>();
            Some(loader.load(
                "tilemaps/map.json",
                JsonFormat,
                self.progress_counter.as_mut().expect("map"),
                &world.read_resource::<AssetStorage<Map>>(),
            ))
        };

        let camera_subject = load_camera_subject(world);
        load_camera(world, camera_subject);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            // Check if all data has been loaded
            if progress_counter.is_complete() {
                // Get the map, which is loaded in the on_start function of load state.
                let map = {
                    let map_storage = &data.world.read_resource::<AssetStorage<Map>>();
                    let map_handle = &self.map_handle.take().unwrap();
                    map_storage.get(map_handle).unwrap().clone()
                };
                let ctx = *data.world.read_resource::<Context>();

                map.load_layers(data.world, &ctx);

                let marine_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Marine).unwrap().clone()
                };
                load_marine(data.world, marine_prefab_handle, &ctx);

                let pincer_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Pincer).unwrap().clone()
                };
                load_pincer(data.world, pincer_prefab_handle, &ctx);

                let flier_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Flier).unwrap().clone()
                };
                load_flier(data.world, flier_prefab_handle, &ctx);
                self.progress_counter = None;
            }
        }
        Trans::None
    }
}
