use std::ops::*;

use sfml::graphics::Color;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + t * b
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub const ZERO: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
    pub fn distance_to_origin(&self) -> f64 {
        self.distance(Point::ZERO)
    }

    pub fn to_color_u8(self) -> Color {
        // self = self * 255.0;
        Color::rgb(
            (self.x % 255.0) as u8,
            (self.y % 255.0) as u8,
            (self.z % 255.0) as u8,
        )
    }
    pub fn to_color_f64(mut self) -> Color {
        self = self * 255.0;
        Color::rgb(
            (self.x % 255.0) as u8,
            (self.y % 255.0) as u8,
            (self.z % 255.0) as u8,
        )
    }
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        (
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
            .to_point()
    }
    pub fn normalized(self) -> Self {
        self / (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)) as f64
    }
}

pub fn lerp_points(a: Point, b: Point, t: Point) -> Point {
    (
        lerp(a.x, b.x, t.x),
        lerp(a.y, b.y, t.y),
        lerp(a.z, b.z, t.z),
    )
        .to_point()
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        (-self.x, -self.y, -self.z).to_point()
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<T: Copy + Mul<f64, Output = f64>> Mul<T> for Point {
    type Output = Point;

    fn mul(self, other: T) -> Self::Output {
        Point::new(other * self.x, other * self.y, other * self.z)
    }
}

impl<T> Div<T> for Point
where
    f64: Div<T, Output = f64>,
    T: Copy,
{
    type Output = Point;

    fn div(self, other: T) -> Self::Output {
        Point::new(self.x / other, self.y / other, self.z / other)
    }
}

pub trait ToPoint {
    fn to_point(&self) -> Point;
}
// impl ToPoint for (f64, f64, f64) {
//     fn to_point(&self) -> Point {
//         Point {
//             x: self.0,
//             y: self.1,
//             z: self.2,
//         }
//     }
// }
impl<A: Into<f64> + Clone, B: Into<f64> + Clone, C: Into<f64> + Clone> ToPoint for (A, B, C) {
    fn to_point(&self) -> Point {
        Point {
            x: self.0.clone().into(),
            y: self.1.clone().into(),
            z: self.2.clone().into(),
        }
    }
}
