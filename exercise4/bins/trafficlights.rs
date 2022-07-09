use exercise04::trafficlight::*;
use std::{env, str::FromStr};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if let Some(x) = args.get(1) {
        match TrafficLight::from_str(x.trim()) {
            Ok(color) => println!("The {} light time is: {}", color, color.get_light_time()),
            Err(err) => println!("{}", err),
        };
    } else {
        println!("The default traffic light time is showing below :");
        println!(
            "The {} light time is: {}",
            TrafficLight::Red,
            TrafficLight::Red.get_light_time()
        );
        println!(
            "The {} light time is: {}",
            TrafficLight::Green,
            TrafficLight::Green.get_light_time()
        );
        println!(
            "The {} light time is: {}",
            TrafficLight::Yellow,
            TrafficLight::Yellow.get_light_time()
        );
    }
}
