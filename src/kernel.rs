use alloc::boxed::Box;
use crate::point::Point;
use crate::bound::Bound;
use crate::octree::Octant;

pub enum KernelT {
    Circle,
    Sphere,
    Square,
    Cube
}


pub struct KernelFactory;

impl KernelFactory {
    pub fn new(p: &Point, r: f64, kt: &KernelT) -> Box<dyn AbstractKernel> {
        match kt {
            KernelT::Circle => Box::new(CircularKernel
            {
                k: Kernel {
                    radius: r,
                    center: Point { x: p.x, y: p.y, z: p.z,id: p.id },
                    inner: Bound::new(p, r * f64::sqrt(2.0) * 0.5),
                    outer: Bound::new(p, r),
                }
            }),
            KernelT::Square => Box::new(SquareKernel
            {
                k: Kernel {
                    radius: r,
                    center: Point { x: p.x, y: p.y, z: p.z,id: p.id },
                    inner: Bound::new(p, r),
                    outer: Bound::new(p, r),
                }
            }),
            KernelT::Cube => Box::new(CubeKernel
            {
                k: Kernel {
                    radius: r,
                    center: Point { x: p.x, y: p.y, z: p.z,id: p.id },
                    inner: Bound::new(p, r),
                    outer: Bound::new(p, r),
                }
            }),            
            KernelT::Sphere => Box::new(SphereKernel
            {
                k: Kernel {
                    radius: r,
                    center: Point { x: p.x, y: p.y, z: p.z,id: p.id },
                    inner: Bound::new(p, r / f64::sqrt(2.0)),
                    outer: Bound::new(p, r),
                }
            }),
        }
    }
}

pub struct Kernel {
    pub radius: f64,
    pub center: Point,
    pub inner: Bound,
    pub outer: Bound,
}

trait Kernel2D {
    fn contains(k: &Kernel, oct: &Box<Octant>) -> bool {
        return oct.bound.min.x > k.inner.min.x && oct.bound.min.y > k.inner.min.y &&
            oct.bound.max.x < k.inner.max.x && oct.bound.max.y < k.inner.max.y;
    }

    fn box_overlap(k: &Kernel, oct: &Box<Octant>) -> bool {
        if oct.bound.max.x < k.outer.min.x || oct.bound.max.y < k.outer.min.y { return false; }
        if oct.bound.min.x > k.outer.max.x || oct.bound.min.y > k.outer.max.y { return false; }

        return true;
    }
}

trait Kernel3D {
    fn contains(k: &Kernel, oct: &Box<Octant>) -> bool {
        return oct.bound.min.x > k.inner.min.x && oct.bound.min.y > k.inner.min.y && oct.bound.min.z > k.inner.min.z &&
            oct.bound.max.x < k.inner.max.x && oct.bound.max.y < k.inner.max.y && oct.bound.max.z < k.inner.max.z;
    }

    fn box_overlap(k: &Kernel, oct: &Box<Octant>) -> bool {
        if oct.bound.max.x < k.outer.min.x || oct.bound.max.y < k.outer.min.y ||
            oct.bound.max.z < k.outer.min.z { return false; }
        if oct.bound.min.x > k.outer.max.x || oct.bound.min.y > k.outer.max.y ||
            oct.bound.min.z > k.outer.max.z { return false; }

        return true;
    }
}


pub trait AbstractKernel {
    fn is_inside(&self, p: &Point) -> bool;
    fn contains(&self, oct: &Box<Octant>) -> bool;
    fn box_overlap(&self, oct: &Box<Octant>) -> bool;
    fn get_center(&self) -> Point;
}

struct CircularKernel {
    k: Kernel,
}

impl Kernel2D for CircularKernel {}
impl AbstractKernel for CircularKernel {
    fn is_inside(&self, p: &Point) -> bool {
        return (p.x - self.k.center.x) * (p.x - self.k.center.x) + (p.y - self.k.center.y) * (p.y - self.k.center.y) < self.k.radius * self.k.radius;
    }

    fn contains(&self, oct: &Box<Octant>) -> bool {
        return <CircularKernel as Kernel2D>::contains(&self.k, oct);
    }

    fn box_overlap(&self, oct: &Box<Octant>) -> bool {
        return <CircularKernel as Kernel2D>::box_overlap(&self.k, oct);
    }

    fn get_center(&self) -> Point {
        return Point::new_from_point(&self.k.center);
    }
}

struct SphereKernel {
    k: Kernel,
}

impl Kernel3D for SphereKernel {}
impl AbstractKernel for SphereKernel {
    fn is_inside(&self, p: &Point) -> bool {
        return (p.x - self.k.center.x) * (p.x - self.k.center.x) + (p.y - self.k.center.y) * (p.y - self.k.center.y) + (p.z - self.k.center.z) * (p.z - self.k.center.z) < self.k.radius * self.k.radius;
    }

    fn contains(&self, oct: &Box<Octant>) -> bool {
        return <SphereKernel as Kernel3D>::contains(&self.k, oct);
    }

    fn box_overlap(&self, oct: &Box<Octant>) -> bool {
        return <SphereKernel as Kernel3D>::box_overlap(&self.k, oct);
    }

    fn get_center(&self) -> Point {
        return Point::new_from_point(&self.k.center);
    }
}


struct SquareKernel {
    k: Kernel,
}

impl Kernel2D for SquareKernel {}
impl AbstractKernel for SquareKernel {
    fn is_inside(&self, p: &Point) -> bool {
        return p.x > self.k.inner.min.x && p.y > self.k.inner.min.y && p.x < self.k.inner.max.x && p.y < self.k.inner.max.y;
    }

    fn contains(&self, oct: &Box<Octant>) -> bool {
        return <SquareKernel as Kernel2D>::contains(&self.k, oct);
    }

    fn box_overlap(&self, oct: &Box<Octant>) -> bool {
        return <SquareKernel as Kernel2D>::box_overlap(&self.k, oct);
    }

    fn get_center(&self) -> Point {
        return Point::new_from_point(&self.k.center);
    }

}


struct CubeKernel {
    k: Kernel,
}

impl Kernel3D for CubeKernel {}
impl AbstractKernel for CubeKernel {
    fn is_inside(&self, p: &Point) -> bool {
        return p.x > self.k.inner.min.x && p.y > self.k.inner.min.y && p.z > self.k.inner.min.z && p.x < self.k.inner.max.x && p.y < self.k.inner.max.y && p.z < self.k.inner.max.z;
    }

    fn contains(&self, oct: &Box<Octant>) -> bool {
        return <CubeKernel as Kernel3D>::contains(&self.k, oct);
    }

    fn box_overlap(&self, oct: &Box<Octant>) -> bool {
        return <CubeKernel as Kernel3D>::box_overlap(&self.k, oct);
    }

    fn get_center(&self) -> Point {
        return Point::new_from_point(&self.k.center);
    }
    
}
