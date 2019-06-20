use amethyst:: {
    assets::{Asset, Handle, ProcessingState},
    core::Transform,
    ecs::{prelude::World, VecStorage},
    error::Error,
    prelude::Builder,
    renderer::{
        sprite::{SpriteRender, SpriteSheetFormat, SpriteSheetHandle},
        transparent::Transparent,
    },
};

use serde::{Deserialize, Serialize};

use specs_physics::{math::Vector3};

use crate::{
    SCALE,
    resources::{AssetType, SpriteSheetList},
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
    pub fn load_non_collision_layer(
        &self,
        world: &mut World,
        // map: &Map,
        asset_type: AssetType,
        z_transform: f32,
        screen_height: f32
    ) {
        let sprite_sheet_handle: SpriteSheetHandle;
        {
            let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
            sprite_sheet_handle = sprite_sheet_list.get(asset_type).unwrap().clone();
        }

        let layer_name = match asset_type {
            AssetType::Background => "background",
            AssetType::Platform => "platform",
            AssetType::Truss => "truss",
        };

        let layer = self.layers.iter()
            .find(|layer| layer.name == layer_name).unwrap();

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