// use amethyst::{
//     core::{math::Vector3, Transform},
//     ecs::prelude::World,
//     prelude::Builder,
//     renderer::{
//         sprite::SpriteSheetHandle,
//         SpriteRender,
//         transparent::Transparent,
//     },
// };

// use crate::{
//     SCALE, BG_Z_TRANSFORM,
// };

// #[derive(Default)]
// pub struct Background {
//     pub width: u32,
//     pub height: u32,
//     pub image_width: f32,
//     pub image_height: f32,
//     pub tile_width: u32,
//     pub tile_height: u32,
// }

// // pub fn load_background(world: &mut World, sprite_sheet: SpriteSheetHandle) {
// //     let background_image = &map.tilesets[0].images[0];
// //     let tile_count = (map.width * map.tile_width) as i32 / background_image.width as i32;

// //     for i in 0..tile_count {
// //         let mut transform = Transform::default();
// //         transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));
// //         let sprite = SpriteRender {
// //             sprite_sheet: sprite_sheet.clone(),
// //             sprite_number: 0,
// //         };

// //         transform.set_translation_xyz(
// //             (i as f32 + 0.5) * background_image.width as f32 * SCALE,
// //             0.5 * background_image.height as f32 * SCALE,
// //             BG_Z_TRANSFORM
// //         );

// //         world.create_entity()
// //             .with(transform)
// //             .with(sprite)
// //             .with(Transparent)
// //             .build();
// //     }
// // }
