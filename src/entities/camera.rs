use amethyst::{
    core::Transform,
    core::math::Matrix4,
    ecs::prelude::World,
    prelude::Builder,
    renderer::camera::Camera,
    window::ScreenDimensions,
};

pub fn load_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    let x = 384.0;
    let y = 176.0;
    transform.set_translation_xyz(x, y, 1.);

    let mut camera = init_parallax_camera(width, height);
    set_paralax_offset(&mut camera, x, y);

    world
        .create_entity()
        .with(camera)
        .with(transform)
        .build();
}

fn init_parallax_camera(width: f32, height: f32) -> Camera{
    let mut camera = Camera::standard_2d(width, height);
    // Set orthogonal projection matrix
    let mut matrix = Matrix4::identity();
    let z_near = 0.01;
    let z_far = 2000.0;

    matrix[(0, 0)] = 2.0/width;
    matrix[(1, 1)] = -2.0/height;
    matrix[(2, 2)] = -1.0 / (z_far - z_near);
    matrix[(2, 3)] = -z_near / (z_far - z_near);

    *camera.as_matrix_mut() = matrix;
    set_paralax_offset(&mut camera, 0.0, 0.0);
    camera
}

/// Add parallax translation to camera.
///
/// Camera assumed to be setup with `init_parallax_camera`
/// (Currently equal to `Camera::standard_2d`)
///
/// If parallax translation being kept equal to camera translation
/// it would cause further objects to move away slower (in screen coordinates)
/// with infinitely distant objects being stationary (infinity=z_far=2000.0)
pub fn set_paralax_offset(camera: &mut Camera, x: f32, y: f32){
    let z_0 = 1.0; // distance at which parallax has no effect
    let z_inf = 200.0; // distance at which parallax would fully apply (infinity plane)

    let matrix = camera.as_matrix_mut();
    let wmod = matrix[(0, 0)]; // wmod = 2/w
    let hmod = matrix[(1, 1)]; // hmod = -2/h
    let zmod = 1.0/(z_inf - z_0); // zmod = 1/infinity, where infinity - depth of unmoving layer
    matrix[(0, 2)] =  wmod * zmod * (-x);
    matrix[(1, 2)] =  hmod * zmod * (-y);
    matrix[(0, 3)] = wmod * zmod * (-x) * z_0;
    matrix[(1, 3)] = hmod * zmod * (-y) * z_0;
}
