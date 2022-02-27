use std::fmt::{Debug, Display};

use crate::parse_input::{Contributor, Project};

pub struct Solution<'a> {
    pub executed_projects: Vec<ExecutedProject<'a>>,
    pub estimated_score: u32,
}

pub struct ExecutedProject<'a> {
    pub project: &'a Project,
    pub contributors: Vec<&'a Contributor>,
}

impl<'a> Display for Solution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n", self.executed_projects.len()))?;
        for executed_project in self.executed_projects.iter() {
            f.write_fmt(format_args!("{}\n", executed_project.project.name))?;
            f.write_fmt(format_args!(
                "{}\n",
                executed_project
                    .contributors
                    .iter()
                    .map(|c| c.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            ))?;
        }

        Ok(())
    }
}

impl<'a> Debug for Solution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Executed projects = {} (estimated score {})",
            self.executed_projects.len(),
            self.estimated_score
        ))
    }
}
