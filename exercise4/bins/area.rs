use exercise04::area::*;
use std::env;
fn main() {
    let args = env::args().map(|s| s.to_lowercase()).collect::<Vec<_>>();
    if args.len() < 3 {
        println!("{} SHAPE [p0 p1 ...]", args[0]);
        return;
    }
    match args[1].as_str() {
        "circle" => {
            let radius = args[2].parse::<f64>().expect("invalid radius");
            let circle = Circle::new(radius);
            if !circle.is_valid() {
                println!("invalid shape {:?}", circle);
                return;
            }
            println!("The area of {:?} is : {}", circle, get_area(&circle));
        }
        "triangle" => {
            if args.len() < 5 {
                println!("need for edges for a triangle");
                return;
            }
            let l1 = args[2].parse::<f64>().expect("invalid length of a edge");
            let l2 = args[3].parse::<f64>().expect("invalid length of a edge");
            let l3 = args[4].parse::<f64>().expect("invalid length of a edge");

            let triangle = Triangle::new(l1, l2, l3);
            if !triangle.is_valid() {
                println!("invalid shape {:?}", triangle);
                return;
            }
            println!("The area of {:?} is : {}", triangle, get_area(&triangle));
        }
        "square" => {
            let l = args[2].parse::<f64>().expect("invalid length of a edge");
            let square = Square::new(l);
            if !square.is_valid() {
                println!("invalid shape {:?}", square);
                return;
            }
            println!("The area of {:?} is : {}", square, get_area(&square));
        }
        _ => {
            println!("Invalid shape {}", args[1]);
        }
    };
}
