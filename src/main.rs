#![allow(dead_code)]
pub mod point;
pub mod ray;
pub mod renderer;
pub mod shape;
pub mod world;

use std::{
    f64::consts::PI,
    sync::{Arc, RwLock},
    vec,
};

use point::*;
use renderer::*;
use sfml::graphics::Color;
use shape::Sphere;
use world::World;

fn main() {
    
    let spehere = Sphere::new((0, 0, 0).to_point(), 3.0, Color::rgb(0, 0, 100));
    let spehere1 = Sphere::new((10, 0, 0).to_point(), 3.0, Color::rgb(100, 0, 0));
    let s = Screen::new(
        Camera {
            origin: (0, 0, -10).to_point(),
            rot_x: 0.0,
            rot_y: 0.0,
            fov: 0.5 * PI,
        },
        Arc::new(RwLock::new(World::new(vec![
            Arc::new(spehere),
            Arc::new(spehere1),
        ]))),
    );
    s.run();
}
