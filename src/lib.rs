use rand::Rng;

use std::io::{Write, stdin, stdout};
use std::str::FromStr;

pub fn input<T: FromStr>(prompt: &str) -> Result<T, T::Err> {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut output = String::new();
    stdin().read_line(&mut output).unwrap();
    output.trim().parse()
}

pub fn random(from: i32, to: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(from..=to)
}

pub fn random_float(from: f32, to: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(from..=to)
}
