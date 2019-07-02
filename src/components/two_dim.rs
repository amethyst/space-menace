use amethyst::{
    core::{Transform},
    ecs::{Component, DenseVecStorage},
};

pub struct TwoDimVector<T> {
    pub x: T,
    pub y: T,
}

impl Default for TwoDimVector<f32> {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

pub struct TwoDimObject {
    pub size: TwoDimVector<f32>,
    pub position: TwoDimVector<f32>,
}

impl Component for TwoDimObject {
    type Storage = DenseVecStorage<Self>;
}

impl TwoDimObject {
    pub fn new(width: f32, height: f32) -> Self {
        TwoDimObject {
            size: TwoDimVector {x: width, y: height},
            position: TwoDimVector {x: 0., y: 0.},
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = TwoDimVector {x, y};
    }

    pub fn update_transform_position(&self, transform: &mut Transform) {
        transform.set_translation_x(self.position.x);
        transform.set_translation_y(self.position.y);
    }

    pub fn top(&self) -> f32 {
        self.position.y + self.size.y / 2.
    }

    pub fn set_top(&mut self, top: f32) {
        self.position.y = top - self.size.y / 2.;
    }

    pub fn bottom(&self) -> f32 {
        self.position.y - self.size.y / 2.
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.position.y = bottom + self.size.y / 2.;
    }

    pub fn left(&self) -> f32 {
        self.position.x - self.size.x / 2.
    }

    pub fn set_left(&mut self, left: f32) {
        self.position.x = left + self.size.x / 2.;
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.size.x / 2.
    }

    pub fn set_right(&mut self, right: f32) {
        self.position.x = right - self.size.x / 2.;
    }

    pub fn overlapping_x(&self, other: &Self) -> bool {
        self.left() < other.right() && other.left() < self.right()
    }

    pub fn overlapping_y(&self, other: &Self) -> bool {
        self.bottom() < other.top() && other.bottom() < self.top()
    }

    pub fn get_next_right(
        &self,
        two_dim_object: &TwoDimObject,
        old_x: f32,
        mut possible_new_x: f32,
    ) -> f32 {
        if self.overlapping_y(two_dim_object)
            && old_x <= two_dim_object.left()
            && possible_new_x >= two_dim_object.left() {
            // Can't early return here, because we need to consider collision with
            // more than one other object. Don't need to set velocity back to zero here,
            // but could depending on how we want the marine animation to act
            possible_new_x = two_dim_object.left();
        }
        possible_new_x
    }

    pub fn get_next_left(
        &self,
        two_dim_object: &TwoDimObject,
        old_x: f32,
        mut possible_new_x: f32
    ) -> f32 {
        if self.overlapping_y(two_dim_object)
            && old_x >= two_dim_object.right()
            && possible_new_x <= two_dim_object.right() {
            // Can't early return here, because we need to consider collision with
            // more than one other object. Don't need to set velocity back to zero here,
            // but could depending on how we want the marine animation to act
            possible_new_x = two_dim_object.right();
        }
        possible_new_x
    }

    pub fn get_next_top(
        &self,
        two_dim_object: &TwoDimObject,
        old_y: f32,
        mut possible_new_y: f32
    ) -> f32 {
        if self.overlapping_x(two_dim_object)
            && old_y <= two_dim_object.bottom()
            && possible_new_y >= two_dim_object.bottom() {
            // Can't early return here, because we need to consider collision with
            // more than one other object. Don't need to set velocity back to zero here,
            // but could depending on how we want the marine animation to act
            possible_new_y = two_dim_object.bottom();
        }
        possible_new_y
    }

    pub fn get_next_bottom(
        &self,
        two_dim_object: &TwoDimObject,
        old_y: f32,
        mut possible_new_y: f32
    ) -> f32 {
        if self.overlapping_x(two_dim_object)
            && old_y >= two_dim_object.top()
            && possible_new_y <= two_dim_object.top() {
            // Can't early return here, because we need to consider collision with
            // more than one other object. Don't need to set velocity back to zero here,
            // but could depending on how we want the marine animation to act
            possible_new_y = two_dim_object.top();
        }
        possible_new_y
    }
}