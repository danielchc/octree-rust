use crate::point::Point;

#[derive(Clone)]
pub struct Bound {
    pub min: Point,
    pub max: Point
}

impl Bound {
    pub fn new(p: &Point, r: f64) -> Self {
        Self {
            min: Point {
                x: p.x - r,
                y: p.y - r,
                z: p.z - r,
                id: None
            },
            max: Point {
                x: p.x + r,
                y: p.y + r,
                z: p.z + r,
                id: None
            },
        }
    }
}

impl Default for Bound {
    fn default() -> Bound {
        Bound {
            min: Default::default(),
            max: Default::default(),
        }
    }
}