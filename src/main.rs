use std::{collections::HashMap, fs::File, io::Write};

use parse_input::parse_input;

use crate::solvers::greedy;
extern crate rand;

mod parse_input;
mod score;
mod solution;
mod solvers;

fn main() {
    let input_dir = "input_data/";
    let input_filenames = vec![
        "a_an_example.in.txt",
        "b_better_start_small.in.txt",
        "c_collaboration.in.txt",
        "d_dense_schedule.in.txt",
        "e_exceptional_skills.in.txt",
        "f_find_great_mentors.in.txt",
    ];
    let submissions_dir = "submissions/5/";

    let mut all_solution_estimated_scores = HashMap::new();

    for filename in input_filenames {
        println!("Input file = {}", filename);

        let input = parse_input(&mut File::open(input_dir.to_owned() + filename).unwrap()).unwrap();
        let solution = greedy::solve(&input, filename.into(), 20).unwrap();
        println!("{:?}", solution);

        File::create(submissions_dir.to_owned() + filename)
            .unwrap()
            .write_fmt(format_args!("{}", solution))
            .unwrap();

        all_solution_estimated_scores.insert(filename, solution.estimated_score);
    }

    println!("\nSubmission estimated score:\n");

    println!(
        "{}",
        all_solution_estimated_scores
            .iter()
            .map(|entry| format!("{} = {}\n", entry.0, entry.1))
            .reduce(|s1, s2| s1 + &s2)
            .unwrap_or_else(|| String::from("No solutions"))
    );

    println!(
        "Total estimated score = {}",
        all_solution_estimated_scores
            .values()
            .cloned()
            .reduce(|a, b| a + b)
            .unwrap_or(0)
    )
}
