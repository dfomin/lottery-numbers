use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;

fn generate(count: usize, max: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (1..=max).collect();
    vec.shuffle(&mut thread_rng());
    vec.truncate(count);
    vec.sort();
    vec
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        let values = arg
            .split("/")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 2, "Incorrect input arguments");
        let count = values[0];
        let max = values[1];
        println!("{:?}", generate(count, max));
    }
}
