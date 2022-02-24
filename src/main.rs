use std::{
    fs::{read_dir, File},
    io::Write,
};

use parse_input::{parse_input, InputData};
use solution::Solution;

use crate::solvers::greedy;
extern crate rand;

mod parse_input;
mod score;
mod solution;
mod solvers;

fn main() {
    let input_dir = "input_data/";
    let submissions_dir = "submissions/4/";
    let print_solution_mode = &PrintSolutionMode::Debug; // TODO: read from commandline options

    let paths = read_dir(input_dir).unwrap();

    for path in paths {
        let filename_os_str = path.unwrap().file_name();
        let filename = filename_os_str.to_str().unwrap();

        if *print_solution_mode == PrintSolutionMode::Debug {
            println!("Input file = {}", filename);
        }

        let input = parse_input(&mut File::open(input_dir.to_owned() + filename).unwrap()).unwrap();
        let solution = greedy::solve(&input, filename.into(), 20).unwrap();
        print_solution(&input, &solution, print_solution_mode);

        File::create(submissions_dir.to_owned() + filename)
            .unwrap()
            .write_fmt(format_args!("{}", solution))
            .unwrap();
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
