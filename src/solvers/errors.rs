use std::fmt::Debug;

pub struct SolveError {
    message: String,
}

impl Debug for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Error while solving: {}", self.message))
    }
}
