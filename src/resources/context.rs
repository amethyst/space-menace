#[derive(Default)]
pub struct Context {
    pub map_width: f32,
    pub background_width: f32,
    pub x_correction: f32,
    pub y_correction: f32,
    pub marine_idle_width: f32,
    pub background_z_translation: f32,
    pub truss_z_translation: f32,
    pub platform_z_translation: f32,
    pub scale: f32,
    pub background_scale: f32,
}

impl Context {
    pub fn new() -> Self {
        Context {
            map_width: 4608.,
            background_width: 384.,
            x_correction: -(1200. / 2. - 384.), // - (screen_width / 2. - background_width)
            y_correction: -176., // background_height / 2.
            marine_idle_width: 48.,
            background_z_translation: -20.,
            truss_z_translation: -10.,
            platform_z_translation: 0.,
            scale: 2.,
            background_scale: 4.,
        }
    }
}