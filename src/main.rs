use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    thread,
};

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

    // NOTE: solution adapted from https://stackoverflow.com/a/50283931
    let all_solution_estimated_scores = Arc::new(Mutex::new(HashMap::new()));

    #[allow(clippy::needless_collect)]
    let handles = input_filenames
        .iter()
        .cloned()
        .map(|filename| {
            let all_solution_estimated_scores = all_solution_estimated_scores.clone();
            thread::spawn(move || {
                println!("Input file = {}", filename);

                let input =
                    parse_input(&mut File::open(input_dir.to_owned() + filename).unwrap()).unwrap();
                let solution = greedy::solve(&input, filename.into(), 20).unwrap();
                println!("{:?}", solution);

                File::create(submissions_dir.to_owned() + filename)
                    .unwrap()
                    .write_fmt(format_args!("{}", solution))
                    .unwrap();

                all_solution_estimated_scores
                    .lock()
                    .unwrap()
                    .insert(filename, solution.estimated_score);
            })
        })
        // NOTE: this is needed since the `map` operator is lazy (thus will not actually spawn the threads unless consumed)
        .collect::<Vec<_>>();

    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .for_each(drop);

    let all_solution_estimated_scores = all_solution_estimated_scores.lock().unwrap();

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
