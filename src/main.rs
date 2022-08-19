#![allow(dead_code)]
pub mod point;
pub mod ray;
pub mod renderer;
pub mod shape;
pub mod world;

use std::f64::consts::PI;

use point::*;
use renderer::*;
use shape::Sphere;

fn main() {
    let spehere = Sphere::new((0, 0, 0).to_point(), 3.0);
    let s = Screen::new(
        spehere,
        Camera {
            origin: (0, 0, -10).to_point(),
            rot_x: 0.0,
            rot_y: 0.0,
            fov: 0.5 * PI,
        },
    );
    s.run();
}
