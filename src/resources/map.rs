use amethyst:: {
    assets::{Asset, Handle, ProcessingState},
    core::Transform,
    ecs::{prelude::World, VecStorage},
    error::Error,
    prelude::Builder,
    renderer::{
        sprite::{SpriteRender, SpriteSheetHandle},
        transparent::Transparent,
    },
    window::ScreenDimensions,
};

use serde::{Deserialize, Serialize};

use specs_physics::{math::Vector3};

use crate::{
    components::TwoDimObject,
    resources::{AssetType, SpriteSheetList},
    SCALE,
};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tileset {
    pub image: String,
    pub imagewidth: i32,
    pub imageheight: i32,
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub rotation: f32,
    pub visible: bool,
    pub x: f32,
    pub y: f32,
    pub properties: Vec<Property>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Layer {
    pub name: String,
    pub opacity: f32,
    pub visible: bool,
    pub x: f32,
    pub y: f32,
    pub objects: Vec<Object>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Property {
    pub name: String,
    pub proptype: String,
    pub value: usize,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub tilesets: Vec<Tileset>,
    pub layers: Vec<Layer>,
}

impl Asset for Map {
    const NAME: &'static str = "code_name_dune::Map";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Map>>;
}

impl From<Map> for Result<ProcessingState<Map>, Error> {
    fn from(map: Map)
        -> Result<ProcessingState<Map>, Error> {
            Ok(ProcessingState::Loaded(map))
        }
}

impl Map {
    pub fn load_layers(&self, world: &mut World) {
        for layer in self.layers.iter() {
            match layer.name.as_ref() {
                "collision" => {
                    self.collision_layer(world, layer);
                },
                _ => {
                    self.load_non_collision_layer(world, layer);
                }
            }
        }
    }

    fn collision_layer(&self, world: &mut World, layer: &Layer) {
        let screen_height = {
            let dim = world.read_resource::<ScreenDimensions>();
            dim.height()
        };
        for obj in layer.objects.iter() {
            let mut transform = Transform::default();
            transform.set_translation_z(-10.);

            let mut two_dim_object = TwoDimObject::new(obj.width * SCALE, obj.height * SCALE);
            two_dim_object.set_left(obj.x * SCALE);
            two_dim_object.set_top(screen_height / 2. - (obj.y * SCALE));
            two_dim_object.update_transform_position(&mut transform);
            println!("{} x-coord = {}", layer.name, obj.x * SCALE);
            println!("{} y-coord = {}", layer.name, screen_height - (obj.y * SCALE));

            world.create_entity()
                .with(transform)
                .with(two_dim_object)
                .build();
        }
    }

    fn load_non_collision_layer(&self, world: &mut World, layer: &Layer) {
        let screen_height = {
            let dim = world.read_resource::<ScreenDimensions>();
            dim.height()
        };
        let mut asset_type = None;
        let mut z_transform = 0.;
        match layer.name.as_ref() {
            "background" => {
                asset_type = Some(AssetType::Background);
                z_transform = -30.;
            },
            "platform" => {
                asset_type = Some(AssetType::Platform);
                z_transform = -10.;
            },
            "truss" => {
                asset_type = Some(AssetType::Truss);
                z_transform = -20.;
            },
            _ => {},
        };
        let sprite_sheet_handle: SpriteSheetHandle;
        {
            let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
            sprite_sheet_handle = sprite_sheet_list.get(asset_type.unwrap()).unwrap().clone();
        }

        for obj in layer.objects.iter() {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                (obj.x + obj.width / 2.) * SCALE,
                screen_height / 2. - (obj.y + obj.height / 2.) * SCALE,
                z_transform
            );

            println!("{} x-coord = {}", layer.name, (obj.x + obj.width / 2.) * SCALE);
            println!("{} y-coord = {}", layer.name, screen_height / 2. - (obj.y + obj.height / 2.) * SCALE);

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