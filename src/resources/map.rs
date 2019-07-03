use amethyst:: {
    assets::{Asset, Handle, ProcessingState},
    core::Transform,
    ecs::{prelude::World, VecStorage},
    error::Error,
    prelude::Builder,
    renderer::{
        sprite::SpriteRender,
        transparent::Transparent,
    },
    window::ScreenDimensions,
};

use serde::{Deserialize, Serialize};

use specs_physics::{math::Vector3};

use crate::{
    components::TwoDimObject,
    resources::{AssetType, Context, SpriteSheetList},
};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Property {
    pub name: String,
    pub value: usize,
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
    pub properties: Option<Vec<Property>>,
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
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub layers: Vec<Layer>,
}

impl Asset for Map {
    const NAME: &'static str = "code_name_dune::Map";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Map>>;
}

impl From<Map> for Result<ProcessingState<Map>, Error> {
    fn from(map: Map) -> Result<ProcessingState<Map>, Error> {
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
        let (scale, x_correction, y_correction) = {
            let context = world.read_resource::<Context>();
            (context.scale, context.x_correction, context.y_correction)
        };

        for obj in layer.objects.iter() {
            let mut transform = Transform::default();
            transform.set_translation_z(-10.);

            let mut two_dim_object = TwoDimObject::new(obj.width * scale, obj.height * scale);
            two_dim_object.set_left(obj.x * scale + x_correction);
            two_dim_object.set_top(screen_height / 2. - (obj.y * scale) + y_correction);
            two_dim_object.update_transform_position(&mut transform);

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
        let (
            x_correction,
            y_correction,
            scale,
            background_scale,
            background_z_translation,
            truss_z_translation,
            platform_z_translation,
        ) = {
            let context = world.read_resource::<Context>();    
            (
                context.x_correction,
                context.y_correction,
                context.scale,
                context.background_scale,
                context.background_z_translation,
                context.truss_z_translation,
                context.platform_z_translation,
            )
        };

        let mut asset_type = None;
        let mut z_translation = 0.;

        match layer.name.as_ref() {
            "background" => {
                asset_type = Some(AssetType::Background);
                z_translation = background_z_translation;
            },
            "platform" => {
                asset_type = Some(AssetType::Platform);
                z_translation = platform_z_translation;
            },
            "truss" => {
                asset_type = Some(AssetType::Truss);
                z_translation = truss_z_translation;
            },
            _ => {},
        };

        let sprite_sheet_handle = {
            let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
            sprite_sheet_list.get(asset_type.unwrap()).unwrap().clone()
        };

        for obj in layer.objects.iter() {
            let mut transform = Transform::default();
            match layer.name.as_ref() {
                "background" |
                "truss" => {
                    transform.set_translation_xyz(
                        (obj.x + obj.width / 2.) * scale + x_correction,
                        screen_height / 2. - (obj.y + obj.height / 2.),
                        z_translation,
                    );
                    transform.set_scale(Vector3::new(background_scale, background_scale, background_scale));
                },
                "platform" => {
                    transform.set_translation_xyz(
                        (obj.x + obj.width / 2.) * scale + x_correction,
                        screen_height / 2. - (obj.y + obj.height / 2.) * scale + y_correction,
                        z_translation,
                    );
                    transform.set_scale(Vector3::new(scale, scale, scale));
                },
                _ => {},
            };

            let sprite_index_prop = match &obj.properties {
                Some(props) => props.iter().find(
                    |prop| prop.name == "spriteindex"
                ),
                None => None
            };
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