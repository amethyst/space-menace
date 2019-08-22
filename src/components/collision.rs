use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage, NullStorage},
};

// use crate::components::BoundingBox;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Boundary {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Boundary {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[derive(Clone)]
pub struct GenericBox {
    pub half_size: Vector2<f32>,
    // pub center: Vector2<f32>,
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
}

impl Default for GenericBox {
    fn default() -> Self {
        Self {
            half_size: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
        }
    }
}

pub struct CollideeDetails {
    pub name: String,
    pub position: Vector2<f32>,
    pub half_size: Vector2<f32>,
    // pub velocity: Vector2<f32>,
    // pub bounding_box: BoundingBox,
    pub correction: f32,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct CollideeNew {
    pub horizontal: Option<CollideeDetails>,
    pub vertical: Option<CollideeDetails>,
}

impl Default for CollideeNew {
    fn default() -> Self {
        Self {
            horizontal: None,
            vertical: None,
        }
    }
}

impl CollideeNew {
    pub fn set_collidee_details(
        &mut self,
        name: String,
        collider_a: &ColliderNew,
        collider_b: &ColliderNew,
        velocity_a: Vector2<f32>,
        velocity_b: Vector2<f32>,
    ) {
        let mut correction = Vector2::new(0., 0.);
        let speed_sum = Vector2::new((velocity_a.x - velocity_b.x).abs(), (velocity_a.y - velocity_b.y).abs());
        let speed_ratio_a = Vector2::new(velocity_a.x / speed_sum.x, velocity_a.y / speed_sum.y);
        let speed_ratio_b = Vector2::new(velocity_b.x / speed_sum.x, velocity_b.y / speed_sum.y);
        let min_safe_distance = Vector2::new(collider_a.half_size.x + collider_b.half_size.x, collider_a.half_size.y + collider_b.half_size.y);
        let overlap = Vector2::new(min_safe_distance.x - (collider_a.position.x - collider_b.position.x).abs(), min_safe_distance.y - (collider_a.position.y - collider_b.position.y).abs());

        println!("collider_a.old_position.y = {}", collider_a.old_position.y);
        println!("collider_b.old_position.y = {}", collider_b.old_position.y);
        // TODO: Reuse is_overlapping_with logic?
        let x_overlapped = (collider_a.old_position.x - collider_b.old_position.x).abs() < collider_a.half_size.x + collider_b.half_size.x;
        let y_overlapped = (collider_a.old_position.y - collider_b.old_position.y).abs() < collider_a.half_size.y + collider_b.half_size.y;

        println!("x_overlapped = {}", x_overlapped);
        println!("y_overlapped = {}", y_overlapped);
        let same_direction = velocity_a.x * velocity_b.x > 0.;
        let faster = speed_ratio_a.x.abs() > speed_ratio_b.x.abs();
        if !x_overlapped && y_overlapped || (!x_overlapped && !y_overlapped && overlap.x.abs() <= overlap.y.abs()) {
            if !same_direction || same_direction && faster {
                println!("overlap.x = {}", overlap.x);
                println!("speed_ratio_a.x = {}", speed_ratio_a.x);
                correction.x = overlap.x * speed_ratio_a.x;
                println!("correction = {}", correction);
                self.horizontal = Some(CollideeDetails {
                    name,
                    position: collider_b.position,
                    half_size: collider_b.half_size,
                    // velocity: velocity_b,
                    // bounding_box: collider_b.clone(),
                    correction: correction.x,
                });
            }
        } else if x_overlapped && y_overlapped { // Might happen when an entity is added at run time.
            if speed_sum.x != 0. {
                correction.x = overlap.x * speed_ratio_a.x;
            } else {
                correction.x = overlap.x;
            }
            self.horizontal = Some(CollideeDetails {
                name: name.clone(),
                position: collider_b.position,
                half_size: collider_b.half_size,
                // velocity: velocity_b,
                // bounding_box: collider_b.clone(),
                correction: correction.x,
            });
        } else {
            println!("overlap.y = {}", overlap.y);
            println!("speed_ratio_a.y = {}", speed_ratio_a.y);
            correction.y = overlap.y * speed_ratio_a.y;
            self.vertical = Some(CollideeDetails {
                name,
                position: collider_b.position,
                half_size: collider_b.half_size,
                // velocity: velocity_b,
                // bounding_box: collider_b.clone(),
                correction: correction.y,
            });
        }
    }
}

// #[derive(Component, Default)]
// #[storage(NullStorage)]
// pub struct ColliderNew;

#[derive(Clone, Component)]
#[storage(DenseVecStorage)]
pub struct ColliderNew {
    pub half_size: Vector2<f32>,
    // pub center: Vector2<f32>,
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub bounding_box: GenericBox,
    pub on_ground: bool,
    pub hit_box_offset_front: f32,
    pub hit_box_offset_back: f32,
}

impl Default for ColliderNew {
    fn default() -> Self {
        Self {
            half_size: Vector2::new(0., 0.),
            // center: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
            bounding_box: GenericBox::default(),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
        }
    }
}

impl ColliderNew {
    pub fn new(width: f32, height: f32) -> Self {
        ColliderNew {
            half_size: Vector2::new(width / 2., height  / 2.),
            // center: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
            bounding_box: GenericBox::default(),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Vector2::new(x, y);
    }

    pub fn is_overlapping_with(&self, other: &ColliderNew) -> bool {
        if (self.position.x - other.position.x).abs() >= (self.half_size.x + other.half_size.x).abs() {
            false
        } else if (self.position.y - other.position.y).abs() >= (self.half_size.y + other.half_size.y).abs() {
            false
        } else {
            true
        }
    }
}
