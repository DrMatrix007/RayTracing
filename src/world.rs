use std::sync::Arc;

use sfml::graphics::Color;

use crate::{
    ray::Ray,
    shape::{CollisionResult, Shape},
};

#[derive(Clone)]
pub struct World {
    pub objects: Vec<Arc<dyn Shape>>,
}

impl World {
    pub fn new(o: Vec<Arc<dyn Shape>>) -> Self {
        Self {
            objects: o,
         }
    }

    pub fn get_color(&self, r: Ray) -> Color {
        static GREY: Color = Color::rgb(69, 69, 69);
        if let CollisionResult::PointCollision(_, d, c) = self.ray_cast(r) {
            c
        } else {
            GREY
        }
    }
    fn ray_cast(&self, r: Ray) -> CollisionResult {
        self.objects
            .iter()
            .map(|x| x.ray_cast(r))
            .fold(CollisionResult::None, |ans, cur| {
                if let CollisionResult::PointCollision(_, d, _) = cur {
                    if let CollisionResult::PointCollision(_, d1, _) = ans {
                        if d < d1 {
                            cur
                        } else {
                            ans
                        }
                    } else {
                        cur
                    }
                } else {
                    ans
                }
            })
    }
}
