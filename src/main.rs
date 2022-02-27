use std::{fs::File, io::Write};

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

    for filename in input_filenames {
        println!("Input file = {}", filename);

        let input = parse_input(&mut File::open(input_dir.to_owned() + filename).unwrap()).unwrap();
        let solution = greedy::solve(&input, filename.into(), 20).unwrap();
        println!("{:?}", solution);

        File::create(submissions_dir.to_owned() + filename)
            .unwrap()
            .write_fmt(format_args!("{}", solution))
            .unwrap();
    }
}
