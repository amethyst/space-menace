use amethyst:: {
    assets::{Handle, Asset, ProcessingState},
    ecs::VecStorage,
    error::Error,
};

use serde::{Deserialize, Serialize};

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