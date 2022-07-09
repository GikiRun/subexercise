use exercise04::sum::*;
fn main() {
    let arr = std::env::args()
        .skip(1)
        .filter_map(|n| match n.parse::<u32>() {
            Ok(num) => Some(num),
            Err(_) => {
                println!("invalid number: {}", n);
                None
            }
        })
        .collect::<Vec<_>>();

    println!("The sum is {:?}", get_sum_of_array(&arr));
}
