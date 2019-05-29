use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
};


#[derive(Clone)]
pub struct BulletResource {
    pub sprite_sheet: Handle<SpriteSheet>,
}
