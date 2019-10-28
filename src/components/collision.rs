use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

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

impl GenericBox {
    pub fn new(width: f32, height: f32) -> Self {
        GenericBox {
            half_size: Vector2::new(width / 2., height / 2.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
        }
    }
}

pub struct CollideeDetails {
    pub name: String,
    pub position: Vector2<f32>,
    pub half_size: Vector2<f32>,
    pub correction: f32,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collidee {
    pub horizontal: Option<CollideeDetails>,
    pub vertical: Option<CollideeDetails>,
}

impl Default for Collidee {
    fn default() -> Self {
        Self {
            horizontal: None,
            vertical: None,
        }
    }
}

impl Collidee {
    pub fn set_collidee_details(
        &mut self,
        name: String,
        collider_a: &Collider,
        collider_b: &Collider,
        velocity_a: Vector2<f32>,
        velocity_b: Vector2<f32>,
        use_hit_box: bool,
    ) {
        let (box_a, box_b) = if use_hit_box {
            (&collider_a.hit_box, &collider_b.hit_box)
        } else {
            (&collider_a.bounding_box, &collider_b.bounding_box)
        };

        let mut correction = Vector2::new(0., 0.);

        let speed_sum = Vector2::new(
            (velocity_a.x - velocity_b.x).abs(),
            (velocity_a.y - velocity_b.y).abs(),
        );
        let speed_ratio_a = Vector2::new(velocity_a.x / speed_sum.x, velocity_a.y / speed_sum.y);
        let speed_ratio_b = Vector2::new(velocity_b.x / speed_sum.x, velocity_b.y / speed_sum.y);

        let min_safe_distance = Vector2::new(
            box_a.half_size.x + box_b.half_size.x,
            box_a.half_size.y + box_b.half_size.y,
        );
        let overlap = Vector2::new(
            min_safe_distance.x - (box_a.position.x - box_b.position.x).abs(),
            min_safe_distance.y - (box_a.position.y - box_b.position.y).abs(),
        );

        // TODO: Reuse is_overlapping_with logic?
        let x_overlapped = (box_a.old_position.x - box_b.old_position.x).abs()
            < box_a.half_size.x + box_b.half_size.x;
        let y_overlapped = (box_a.old_position.y - box_b.old_position.y).abs()
            < box_a.half_size.y + box_b.half_size.y;

        let same_direction = velocity_a.x * velocity_b.x > 0.;
        let faster = speed_ratio_a.x.abs() > speed_ratio_b.x.abs();
        if (y_overlapped || overlap.x.abs() <= overlap.y.abs()) && !x_overlapped {
            if faster || !same_direction {
                correction.x = overlap.x * speed_ratio_a.x;
            }
            // No correction (correction = 0.) is required if collider is slower
            // and both bodies are moving in the same direction
            self.horizontal = Some(CollideeDetails {
                name,
                position: box_b.position,
                half_size: box_b.half_size,
                correction: correction.x,
            });
        } else if x_overlapped && y_overlapped {
            // Might happen when an entity is added at run time.
            // As per the current game design, no correction (correction = 0.) is required for this scenario
            // This might have to be changed in future
            self.horizontal = Some(CollideeDetails {
                name,
                position: box_b.position,
                half_size: box_b.half_size,
                correction: correction.x,
            });
        } else {
            correction.y = overlap.y * speed_ratio_a.y;
            self.vertical = Some(CollideeDetails {
                name,
                position: box_b.position,
                half_size: box_b.half_size,
                correction: correction.y,
            });
        }
    }
}

#[derive(Clone, Component)]
#[storage(DenseVecStorage)]
pub struct Collider {
    pub bounding_box: GenericBox,
    pub hit_box: GenericBox,
    pub hit_box_offset: Vector2<f32>,
    pub on_ground: bool,
    pub hit_box_offset_front: f32,
    pub hit_box_offset_back: f32,
    pub is_collidable: bool,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            bounding_box: GenericBox::default(),
            hit_box: GenericBox::default(),
            hit_box_offset: Vector2::new(0., 0.),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
            is_collidable: true,
        }
    }
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Collider {
            bounding_box: GenericBox::new(width, height),
            hit_box: GenericBox::new(width, height),
            hit_box_offset: Vector2::new(0., 0.),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
            is_collidable: true,
        }
    }

    pub fn set_hit_box_position(&mut self, velocity: Vector2<f32>) {
        let hbox_position = &mut self.hit_box.position;
        let bbox_position = self.bounding_box.position;
        hbox_position.x = if velocity.x >= 0. {
            bbox_position.x + self.hit_box_offset.x
        } else {
            bbox_position.x - self.hit_box_offset.x
        };
        hbox_position.y = if velocity.y >= 0. {
            bbox_position.y + self.hit_box_offset.y
        } else {
            bbox_position.y - self.hit_box_offset.y
        };
    }

    pub fn is_overlapping_with(&self, other: &Collider, use_hit_box: bool) -> bool {
        let (self_box, other_box) = if use_hit_box {
            (&self.hit_box, &other.hit_box)
        } else {
            (&self.bounding_box, &other.bounding_box)
        };
        !((self_box.position.x - other_box.position.x).abs()
            >= (self_box.half_size.x + other_box.half_size.x).abs()
            || (self_box.position.y - other_box.position.y).abs()
                >= (self_box.half_size.y + other_box.half_size.y).abs())
    }
}
