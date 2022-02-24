use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, Read},
    num,
};

pub fn parse_input<TRead: Read>(reader: &mut TRead) -> Result<InputData, ParseError> {
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let mut lines = content.split('\n');

    let mut skills_translation_map: HashMap<&str, usize> = HashMap::new();

    let split_first_line = lines.next().unwrap().split(' ').collect::<Vec<_>>();
    let num_contributors = split_first_line[0].parse::<usize>()?;
    let num_projects = split_first_line[1].parse::<usize>()?;

    let mut contributors = vec![];
    let mut projects = vec![];

    for _ in 0..num_contributors {
        let contributor_split_first_line = lines.next().unwrap().split(' ').collect::<Vec<_>>();
        let name = contributor_split_first_line[0];
        let num_skills = contributor_split_first_line[1].parse::<usize>()?;
        let mut skills = HashMap::new();

        for _ in 0..num_skills {
            let contributor_skill_split_first_line =
                lines.next().unwrap().split(' ').collect::<Vec<_>>();
            let skill_name = contributor_skill_split_first_line[0];
            let skill_level = contributor_skill_split_first_line[1].parse::<u8>()?;

            let new_skill_id = skills_translation_map.len();
            let skill_id = skills_translation_map
                .entry(skill_name)
                .or_insert(new_skill_id);

            skills.insert(*skill_id, skill_level);
        }

        contributors.push(Contributor {
            name: String::from(name),
            skills,
        });
    }

    for _ in 0..num_projects {
        let project_split_first_line = lines.next().unwrap().split(' ').collect::<Vec<_>>();
        let name = project_split_first_line[0];
        let duration = project_split_first_line[1].parse::<u16>()?;
        let score = project_split_first_line[2].parse::<u16>()?;
        let best_before = project_split_first_line[3].parse::<u16>()?;
        let num_roles = project_split_first_line[4].parse::<u8>()?;
        let mut roles = vec![];

        for _ in 0..num_roles {
            let project_role_split_first_line =
                lines.next().unwrap().split(' ').collect::<Vec<_>>();
            let role_skill_name = project_role_split_first_line[0];
            let role_required_skill_level = project_role_split_first_line[1].parse::<u8>()?;

            let role_skill_id = skills_translation_map.get(role_skill_name).unwrap();

            roles.push(Role {
                skill_id: *role_skill_id,
                required_skill_level: role_required_skill_level,
            });
        }

        projects.push(Project {
            name: String::from(name),
            duration,
            score,
            best_before,
            roles,
        });
    }

    Ok(InputData {
        contributors,
        projects,
    })
}

#[derive(Debug)]
pub struct InputData {
    pub contributors: Vec<Contributor>,
    pub projects: Vec<Project>,
}

#[derive(Debug)]
pub struct Contributor {
    pub name: String,
    pub skills: HashMap<usize, u8>,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub duration: u16,
    pub score: u16,
    pub best_before: u16,
    pub roles: Vec<Role>,
}

#[derive(Debug)]
pub struct Role {
    pub skill_id: usize,
    pub required_skill_level: u8,
}

pub struct ParseError {
    message: String,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Error in parsing input: {}", self.message))
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}
