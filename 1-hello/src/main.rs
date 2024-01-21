fn main() {
    let mut name: &str;
    name = "Rust";
    println!("Hello, {}!", name);
    name = "Alberto Basalo";
    println!("By {}.", name);
    // maximum unsigned integer u128
    let max_u128: u128 = 340282366920938463463374607431768211455;
    println!("The maximum unsigned integer is  {}.", max_u128);
    // maximum signed integer i128
    let max_i128: i128 = 170141183460469231731687303715884105727;
    println!("The maximum + signed integer is  {}.", max_i128);
    // minimum signed integer i128
    let min_i128: i128 = -170141183460469231731687303715884105728;
    println!("The minimum - signed integer is {}.", min_i128);
    // a float min / max
    let min_f64: f64 = min_i128 as f64 / max_u128 as f64;
    println!("The ratio min/max is {}.", min_f64);
    // square root of 2
    let sqrt2: f64 = 2.0_f64.sqrt();
    println!("The square root of 2 is {}.", sqrt2);
    // square root of -1, no imagination
    let sqrt_1: f64 = (-1.0_f64).sqrt();
    println!("The square root of -1 is {}.", sqrt_1);
    // division by zero, don't panic
    let div0: f64 = 1.0_f64 / 0.0_f64;
    println!("The division by zero is {}.", div0);
    // is even or odd, a boolean question
    let is_even: bool = max_u128 % 2 == 0;
    println!("Is the maximum unsigned integer even? : {}.", is_even);
    // working with strings
    let bye: String = String::from("Bye");
    let see_you: &str = "See you soon!";
    let mut bye_see_you: String = bye;
    bye_see_you.push_str(", ");
    bye_see_you.push_str(see_you);
    println!("{}", bye_see_you);
}
