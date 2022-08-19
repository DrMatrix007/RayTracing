use std::sync::Arc;

use sfml::graphics::Color;

use crate::{ray::Ray, shape::{Shape, CollisionResult}};

#[derive(Clone)]
pub struct World {
    pub object:Arc<dyn Shape>
}

impl World {
    pub fn get_color(&self, r: Ray) -> Color {
        static GREY: Color = Color::rgb(69, 69, 69);
        if let CollisionResult::PointCollision(_,c) = self.object.as_ref().ray_cast(r) {
            c
        } else {
            GREY
        }
    }
}
