use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
};

#[derive(Clone)]
pub struct BulletResource {
    pub sprite_sheet: Handle<SpriteSheet>,
}

#[derive(Clone)]
pub struct BulletImpactResource {
    pub sprite_sheet: Handle<SpriteSheet>,
}

