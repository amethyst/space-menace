use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    core::{math::Vector3, Transform, WithNamed},
    ecs::{prelude::World, VecStorage},
    error::Error,
    prelude::Builder,
    renderer::{sprite::SpriteRender, transparent::Transparent},
};

use serde::{Deserialize, Serialize};

use crate::{
    components::{Collidee, Direction, Motion, Parallax, TwoDimObject},
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
    const NAME: &'static str = "space_menace::Map";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Map>>;
}

impl From<Map> for Result<ProcessingState<Map>, Error> {
    fn from(map: Map) -> Result<ProcessingState<Map>, Error> {
        Ok(ProcessingState::Loaded(map))
    }
}

impl Map {
    pub fn load_layers(&self, world: &mut World, ctx: &Context) {
        for layer in self.layers.iter() {
            match layer.name.as_ref() {
                "collision" => {
                    self.collision_layer(world, layer, ctx);
                }
                _ => {
                    self.load_non_collision_layer(world, layer, ctx);
                }
            }
        }
    }

    fn collision_layer(&self, world: &mut World, layer: &Layer, ctx: &Context) {
        let scale = ctx.scale;

        for obj in layer.objects.iter() {
            let mut transform = Transform::default();
            transform.set_translation_z(-10.);

            let mut two_dim_object = TwoDimObject::new(obj.width * scale, obj.height * scale);
            two_dim_object.set_left(obj.x * scale + ctx.x_correction);
            two_dim_object.set_top(ctx.bg_height * 2. - (obj.y * scale) + ctx.y_correction);
            two_dim_object.update_transform_position(&mut transform);

            world
                .create_entity()
                .named("Collision")
                .with(Motion::new())
                .with(transform)
                .with(two_dim_object)
                .with(Collidee::default())
                .with(Direction::default())
                .build();
        }
    }

    fn load_non_collision_layer(&self, world: &mut World, layer: &Layer, ctx: &Context) {
        let scale = ctx.scale;
        let x_correction = ctx.x_correction;

        let mut asset_type_wrapper = None;
        let mut z_translation = 0.;

        match layer.name.as_ref() {
            "background" => {
                asset_type_wrapper = Some(AssetType::Background);
                z_translation = ctx.bg_z_translation;
            }
            "platform" => {
                asset_type_wrapper = Some(AssetType::Platform);
                z_translation = ctx.platform_z_translation;
            }
            "truss" => {
                asset_type_wrapper = Some(AssetType::Truss);
                z_translation = ctx.truss_z_translation;
            }
            _ => {}
        };

        if let Some(asset_type) = asset_type_wrapper {
            let sprite_sheet_handle = {
                let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
                sprite_sheet_list.get(asset_type).unwrap().clone()
            };

            for obj in layer.objects.iter() {
                let mut transform = Transform::default();

                let sprite_index_prop = match &obj.properties {
                    Some(props) => props.iter().find(|prop| prop.name == "spriteindex"),
                    None => None,
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
                    }
                    None => {}
                }

                match layer.name.as_ref() {
                    "background" | "truss" => {
                        transform.set_translation_xyz(
                            (obj.x + obj.width / 2.) * scale + x_correction,
                            ctx.bg_height * 2. - (obj.y + obj.height / 2.),
                            z_translation,
                        );
                        transform.set_scale(Vector3::new(4., 4., 4.));
                        world
                            .create_entity()
                            .with(transform)
                            .with(sprite)
                            .with(Transparent)
                            .with(Parallax::default())
                            .build();
                    }
                    "platform" => {
                        transform.set_translation_xyz(
                            (obj.x + obj.width / 2.) * scale + x_correction,
                            ctx.bg_height * 2. - (obj.y + obj.height / 2.) * scale
                                + ctx.y_correction,
                            z_translation,
                        );
                        transform.set_scale(Vector3::new(scale, scale, scale));
                        world
                            .create_entity()
                            .with(transform)
                            .with(sprite)
                            .with(Transparent)
                            .build();
                    }
                    _ => {}
                };
            }
        }
    }
}
