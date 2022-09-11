use crate::point::Point;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(o: Point, d: Point) -> Self {
        Self {
            origin: o,
            direction: d,
        }
    }

    // add the origin and the direction times the coefficient
    pub fn get_point_with_coefficient(&self, a: f64) -> Point {
        self.origin + self.direction * a
    }
}
