use std::fs::{read_dir, File};

use parse_input::{parse_input, InputData};
use solution::Solution;
use solvers::example;
extern crate rand;

mod parse_input;
mod score;
mod solution;
mod solvers;

fn main() {
    let input_dir = "input_data/";
    let submissions_dir = "submissions/1/";
    let print_solution_mode = &PrintSolutionMode::Debug; // TODO: read from commandline options

    let paths = read_dir(input_dir).unwrap();

    for path in paths {
        let path_buf = path.unwrap().path();
        let filename = path_buf.to_str().unwrap();
        if *print_solution_mode == PrintSolutionMode::Debug {
            println!("Input file = {}", filename);
        }

        let input = parse_input(&mut File::open(filename).unwrap()).unwrap();
        let solution = example::solve(&input).unwrap();
        print_solution(&input, &solution, print_solution_mode);
    }
}

#[derive(PartialEq)]
enum PrintSolutionMode {
    Submission,
    Debug,
}

fn print_solution(input_data: &InputData, solution: &Solution, mode: &PrintSolutionMode) {
    match mode {
        PrintSolutionMode::Submission => println!("{}", solution),
        PrintSolutionMode::Debug => println!(
            "{:?} (expected score = {})",
            solution,
            score::score(input_data, solution)
        ),
    }
}
