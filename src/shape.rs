use crate::{ray::Ray, point::Point};
#[derive(Debug,Clone, Copy)]
pub enum CollisionResult {
    None,
    PointCollision(Point)
}
pub trait Shape {
    fn ray_cast(&self,r:Ray) -> CollisionResult; 
}