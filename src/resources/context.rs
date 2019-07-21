#[derive(Clone, Copy, Default)]
pub struct Context {
    pub map_width: f32,
    pub bg_width: f32,
    pub bg_height: f32,
    pub x_correction: f32,
    pub y_correction: f32,
    pub bg_z_translation: f32,
    pub truss_z_translation: f32,
    pub platform_z_translation: f32,
    pub scale: f32,
}

impl Context {
    pub fn new() -> Self {
        Context {
            map_width: 4608.,
            bg_width: 384.,
            bg_height: 352.,
            x_correction: -(1200. / 2. - 384.), // - (screen_width / 2. - background_width)
            y_correction: -176.,                // (background_height / 2.) * -1.
            bg_z_translation: -50.,
            truss_z_translation: -40.,
            platform_z_translation: -10.,
            scale: 2.,
        }
    }
}
