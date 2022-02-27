use crate::{parse_input::InputData, solution::Solution};

pub fn score(input: &InputData, solution: &Solution) -> u32 {
    solution.estimated_score as u32
}
