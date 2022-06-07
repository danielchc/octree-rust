use std::ops::Sub;

#[derive(Clone)]
pub struct Point
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: Option<usize>,
}

impl Point {
    pub fn new_from_point(p: &Self) -> Self {
        return Self {
            x: p.x,
            y: p.y,
            z: p.z,
            id: p.id,
        }
    }
}

// Overload - operator // TODO: Do it for any scalar type.
impl Sub<f64> for Point {
    type Output = Point;

    fn sub(self, rhs: f64) -> Point {
        return Point{x: self.x - rhs, y: self.y - rhs, z: self.z - rhs, id:self.id};
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        return Point{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, id: self.id};
    }
}

impl Default for Point {
    fn default() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            id: None,
        }
    }
}