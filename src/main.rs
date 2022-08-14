#![allow(dead_code)]
pub mod point;
pub mod ray;
pub mod renderer;
pub mod shape;

use std::f64::consts::PI;

use point::*;
use ray::Ray;
use renderer::*;
use shape::{CollisionResult, Shape};
struct Sphere {
    origin: Point,
    r: f64,
}
impl Shape for Sphere {
    fn ray_cast(&self, r: Ray) -> CollisionResult {
        let dif = r.origin - self.origin;
        let d = r.direction;
        let a = d.x.powf(2.0) + d.y.powf(2.0) + d.z.powf(2.0);
        let b = 2.0 * (d.x * dif.x + d.y * dif.y + d.z * dif.z);
        let c = dif.x.powf(2.0) + dif.y.powf(2.) + dif.z.powf(2.) - self.r.powf(2.);
        let delta = b * b - 4.0 * a * c;

        if delta < 0.0 {
            // println!("None :( {}", delta);
            return CollisionResult::None;
        }
        let t1 = (-b + delta.sqrt()) / (2.0 * a);
        let t2 = (-b - delta.sqrt()) / (2.0 * a);

        let p1 = r.get_point_with_coefficient(t1);
        let p2 = r.get_point_with_coefficient(t2);

        if p1.distance(r.origin) < p2.distance(r.origin) {
            if t1 > 0.0 {
                CollisionResult::PointCollision(p1)
            } else {
                CollisionResult::None
            }
        } else {
            if t2 > 0.0 {
                CollisionResult::PointCollision(p2)
            } else {
                CollisionResult::None
            }
        }
    }
}
fn main() {
    let spehere = Sphere {
        origin: (0, 0, 0).to_point(),
        r: 3.0,
    };
    println!("{:?}", (0, 0, 1).to_point().cross((1, 0, 0).to_point()));
    let mut s = Screen::new(
        spehere,
        Camera {
            origin: (0, 0, -10).to_point(),
            rot_x: 0.0,
            rot_y: 0.0,
            fov: 90./180. * PI,
        },
    );
    s.run();
}
