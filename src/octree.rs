use alloc::boxed::Box;
use crate::bound::Bound;
use crate::kernel::{AbstractKernel, KernelT, KernelFactory};
use crate::point::Point;


pub struct Octree<'a> {
    pub root: Box<Octant>,
    pub points: Vec<&'a Point>,
    pub successors: Vec<usize>,
    pub bucket_size: usize,
}

impl<'a> Octree<'a> {
    pub fn new(points: &'a Vec<Point>,bucket_size: usize) -> Self {
        println!("Building Octree with {:?} points", points.len());
        let mut oct = Self {
            root: Default::default(),
            successors: Vec::with_capacity(points.len()),
            points: Vec::with_capacity(points.len()),
            bucket_size: bucket_size
        };
        oct.insert_points(points);

        let (center, radius) = oct.mbb();
        oct.build(center, radius);
        return oct;
    }

    fn build(&mut self, center: Point, radius: f64) {
        self.root = self.create_octant(center, radius, 0, self.points.len() - 1, self.points.len());
    }

    fn create_octant(&mut self, center: Point, radius: f64,
                     start_idx: usize, end_idx: usize, size: usize) -> Box<Octant> {
        let mut octant = Box::new(Octant {
            octants: vec![Default::default(); 8],
            center: Point::new_from_point(&center),
            radius: radius,
            start_idx: start_idx,
            end_idx: end_idx,
            bound: Bound::new(&center, radius),
            size: size,
            n_octants: 0,
        });


        if size <= self.bucket_size {
            return octant;
        }

        let mut idx: usize = start_idx;

        let mut child_start: [usize; 8] = [0; 8];
        let mut child_end: [usize; 8] = [0; 8];
        let mut child_sizes: [usize; 8] = [0; 8];

        for _i in 0..size {
            let p = &self.points[idx];
            let morton_code = octant.octant_idx(&p);

            if child_sizes[morton_code] == 0 {
                child_start[morton_code] = idx;
            } else {
                self.successors[child_end[morton_code]] = idx;
            }

            child_sizes[morton_code] += 1;
            child_end[morton_code] = idx;
            idx = self.successors[idx];
        }

        let new_radius = 0.5f64 * radius;
        let mut last_idx = 0usize;
        for i in 0..8 {
            if child_sizes[i] == 0 { continue; };

            let new_center = Point {
                x: octant.center.x + (if i & 1 > 0 { 0.5f64 } else { -0.5f64 }) * radius,
                y: octant.center.y + (if i & 2 > 0 { 0.5f64 } else { -0.5f64 }) * radius,
                z: octant.center.z + (if i & 4 > 0 { 0.5f64 } else { -0.5f64 }) * radius,
                id: None
            };

            octant.octants[i] = self.create_octant(new_center, new_radius,
                                                   child_start[i], child_end[i], child_sizes[i]);

            if octant.n_octants == 0 {
                octant.start_idx = octant.octants[i].start_idx;
            } else {
                self.successors[octant.octants[last_idx].end_idx] = octant.octants[i].start_idx;
            }

            last_idx = i;
            octant.end_idx = octant.octants[i].end_idx;
            octant.n_octants += 1;
        }

        return octant;
    }

    pub fn insert_points(&mut self, points: &'a Vec<Point>) {
        self.successors.resize(points.len(), 0usize);

        let mut index = 0usize;
        for p in points {
            self.insert_point(p);
            self.successors[index] = index + 1;
            index += 1;
        }
    }

    pub fn insert_point(&mut self, p: &'a Point) {
        self.points.push(p);
    }

    fn compute_center(&self, min: &Point, max: &Point) -> (Point, Point)
    {
        let radii = Point {
            x: (max.x - min.x) / 2.0,
            y: (max.y - min.y) / 2.0,
            z: (max.z - min.z) / 2.0,
            id: None
        };

        let center = Point {
            x: min.x + radii.x,
            y: min.y + radii.y,
            z: min.z + radii.z,
            id: None
        };

        return (center, radii);
    }

    fn mbb(&self) -> (Point, f64) {
        let mut min = Point { x: f64::MAX, y: f64::MAX, z: f64::MAX, id: None };
        let mut max = Point { x: f64::MIN, y: f64::MIN, z: f64::MIN, id: None };

        for p in &self.points {
            if p.x < min.x { min.x = p.x };
            if p.y < min.y { min.y = p.y };
            if p.z < min.z { min.z = p.z };
            if p.x > max.x { max.x = p.x };
            if p.y > max.y { max.y = p.y };
            if p.z > max.z { max.z = p.z };
        }

        let (center, radii) = self.compute_center(&min, &max);
        let radius = f64::max(radii.x, f64::max(radii.y, radii.z));

        return (center, radius);
    }

    fn neighbors(&self, oct: &Box<Octant>, k: &Box<dyn AbstractKernel>, pts_inside: &mut Vec<&'a Point>) {
        let mut current;

        // If kernel contains the entire octree
        if k.contains(oct) {
            current = oct.start_idx;
            for _i in 0..oct.size {
                let p = &self.points[current];
                if k.get_center().id.unwrap() != p.id.unwrap() {
                    pts_inside.push(p);
                }
                current = self.successors[current];
            }
            return;
        }

        if oct.is_leaf() {
            current = oct.start_idx;
            for _i in 0..oct.size {
                let p = &self.points[current];
                if k.is_inside(p) && k.get_center().id.unwrap() != p.id.unwrap() {
                    pts_inside.push(p);
                }
                current = self.successors[current];
            }
            return;
        }

        for child in &oct.octants {
            if k.box_overlap(child) {
                self.neighbors(child, k, pts_inside);
            }
        }
    }

    pub fn search_neighbors(&self, p: & Point, r: f64, kt: &KernelT) -> Vec<&Point> {
        let mut pts_inside = Vec::<&Point>::new();
        let kernel = KernelFactory::new(p, r, kt);
        self.neighbors(&self.root, &kernel, &mut pts_inside);

        return pts_inside;
    }


}

impl<'a> Default for Octree<'a> {
    fn default() -> Self {
        Self {
            points: vec![],
            successors: vec![],
            root: Default::default(),
            bucket_size: 25
        }
    }
}

#[derive(Clone)]
pub struct Octant {
    pub octants: Vec<Box<Octant>>,
    pub start_idx: usize,
    pub end_idx: usize,
    pub size: usize,
    pub bound: Bound,
    pub center: Point,
    pub radius: f64,
    pub n_octants: usize,
}

impl Octant {
    fn octant_idx(&self, p: &Point) -> usize {
        let mut child = 0usize;

        if p.x > self.center.x { child |= 1; };
        if p.y > self.center.y { child |= 2; };
        if p.z > self.center.z { child |= 4; };

        return child;
    }
    fn is_leaf(&self) -> bool {
        return self.n_octants == 0;
    }
}

impl Default for Octant {
    fn default() -> Self {
        Self {
            start_idx: 0,
            end_idx: 0,
            size: 0,
            center: Default::default(),
            bound: Default::default(),
            radius: 0.0,
            octants: Vec::with_capacity(8),
            n_octants: 0,
        }
    }
}