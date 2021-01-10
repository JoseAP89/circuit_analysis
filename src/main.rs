use std::env;
use std::fs;
mod circuit;
mod utils;
mod matrix;
use crate::circuit::Circuit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let contents : Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    let circuit = Circuit::new(contents);
    circuit.solve();
}