extern crate alloc;

use las::{Read, Reader};
use std::time::Instant;

mod point;
use point::Point;
mod bound;
mod kernel;
mod octree;

use kernel::KernelT;
use octree::Octree;
use clap::{Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short='i', long,required = true)]
    input: String,    
    
    #[clap(short='b',long,required = true)]
    bucket_size: usize
}


fn main() {
    let cli = Args::parse();
    // Read the data
    let mut reader = Reader::from_path(cli.input).unwrap();

    let mut points = Vec::new();
    let mut index=0;
    for wrapped_point in reader.points() {
        let p = wrapped_point.unwrap();
        points.push(Point {
            x: p.x,
            y: p.y,
            z: p.z,
            id: Some(index)
        });
        index+=1;
    }

    
 
    let now = Instant::now();
    let _oct = Octree::new(&points,cli.bucket_size);
    let elapsed = now.elapsed().as_millis();
    println!("Octree creation {} ms", elapsed);
    println!("Points: {}", _oct.points.len());
	
	
	let _n_points=_oct.points.len();
	let now = Instant::now();
	for i in 0..100 {
		let _neighs = _oct.search_neighbors(&points[i], 1.0, &KernelT::Circle);
	}
	let elapsed = now.elapsed().as_millis();
    println!("Search time {} ms", elapsed);

}