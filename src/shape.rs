use sfml::graphics::Color;

use crate::{point::Point, ray::Ray};
#[derive(Debug, Clone, Copy)]
pub enum CollisionResult {
    None,
    PointCollision(Point, Color),
}
pub trait Shape: 'static + Send + Sync {
    fn ray_cast(&self, r: Ray) -> CollisionResult;
    fn get_color(&self) -> Color;
}
// unsafe impl Send for dyn Shape{}
// unsafe impl Sync for dyn Shape{}

pub struct Sphere {
    origin: Point,
    r: f64,
}
impl Sphere {
    pub fn new(o: Point, r: f64) -> Self {
        Self { origin: o, r }
    }
}
impl Shape for Sphere {
    fn ray_cast(&self, r: Ray) -> CollisionResult {
        let (t1,t2,p);
        let dif = r.origin - self.origin;
        let d = r.direction;
        let a = d.dot(d);
        let b = d.dot(dif);
        let c = dif.dot(dif) - self.r * self.r;
        let delta = b * b - a * c;

        if delta < 0.0 {
            // println!("None :( {}", delta);
            return CollisionResult::None;
        }
        t1 = (-b + delta.sqrt()) / (a);
        t2 = (-b - delta.sqrt()) / (a);

        if t1 > 0.0 && (t2 > t1 || t2 < 0.0) {
            p = r.get_point_with_coefficient(t1);
            CollisionResult::PointCollision(p, self.get_color())
        } else if t2 > 0.0{
            let p = r.get_point_with_coefficient(t2);
            CollisionResult::PointCollision(p, self.get_color())
        } else {
            CollisionResult::None
        }
    }

    fn get_color(&self) -> sfml::graphics::Color {
        // Color::rgb(random(),random(),random())
        Color::RED
    }
}
