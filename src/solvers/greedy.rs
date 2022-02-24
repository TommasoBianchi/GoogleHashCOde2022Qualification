use std::collections::{HashMap, HashSet};

use crate::{
    parse_input::{Contributor, InputData, Project, Role},
    solution::{ExecutedProject, Solution},
};

use super::errors::SolveError;

pub fn solve(input: &InputData) -> Result<Solution, SolveError> {
    let mut available_projects_ids: HashSet<usize> =
        (0..input.projects.len()).into_iter().collect();
    let mut available_contributors_ids: HashSet<usize> =
        (0..input.contributors.len()).into_iter().collect();
    let mut contributors_ids_to_freeup_time: HashMap<usize, u32> = HashMap::new();
    let mut current_time = 0_u32;
    let mut next_current_time = 0_u32;
    let mut executed_projects = vec![];

    let mut current_contributors: Vec<Contributor> = input.contributors.to_vec();

    while !available_projects_ids.is_empty() {
        println!("Remaining projects = {}", available_projects_ids.len());

        // Sort projects by score
        let sorted_projects =
            sort_projects(input, current_time, available_projects_ids.iter().cloned());

        let mut assigned_projects = 0;

        for project_id in sorted_projects {
            let project = &input.projects[project_id];

            // Assign contributors to the project
            let contributors_option = assign_contributors(
                &current_contributors,
                project,
                &mut available_contributors_ids,
            );

            if contributors_option.is_none() {
                continue;
            }

            // Update next current time
            if next_current_time == current_time {
                next_current_time += project.duration;
            } else {
                next_current_time = next_current_time.min(current_time + project.duration);
            }

            let contributors_ids = contributors_option.unwrap();

            // Store freeup time and increase skills for each contributor
            for contributor_data in contributors_ids.iter() {
                let contributor_id = contributor_data.0;
                let skill_id = contributor_data.1;
                let required_skill_level = contributor_data.2;

                contributors_ids_to_freeup_time
                    .insert(contributor_id, current_time + project.duration);

                // Increase skill level for contributors (based on the skill they contributed for)
                let skills = &mut current_contributors[contributor_id].skills;
                let current_skill_level = skills.get_mut(&skill_id).unwrap();
                if required_skill_level <= *current_skill_level {
                    *current_skill_level += 1;
                }
            }

            assigned_projects += 1;

            // Save results
            executed_projects.push(ExecutedProject {
                project,
                contributors: contributors_ids
                    .iter()
                    .map(|contributor_data| &input.contributors[contributor_data.0])
                    .collect(),
            });

            // Delete best project from available ones
            available_projects_ids.remove(&project_id);

            // TODO: consider mentoring (skills level up)
        }

        current_time = next_current_time.max(current_time + 1);

        println!(
            "Assigned projects = {} this round, {} total; next time = {}; remaining projects = {}",
            assigned_projects,
            executed_projects.len(),
            current_time,
            available_projects_ids.len()
        );

        // Free up contributors
        let contributors_ids_to_free = contributors_ids_to_freeup_time
            .iter()
            .filter(|entry| *entry.1 >= current_time)
            .map(|entry| entry.0)
            .cloned()
            .collect::<Vec<_>>();
        for contributor_id in contributors_ids_to_free.iter() {
            contributors_ids_to_freeup_time.remove(contributor_id);
            available_contributors_ids.insert(*contributor_id);
        }

        // Cleanup projects no longer doable
        for project_id in available_projects_ids.iter().cloned().collect::<Vec<_>>() {
            let project = &input.projects[project_id];
            if current_time >= project.best_before + project.score {
                available_projects_ids.remove(&project_id);
            }
        }
    }

    Ok(Solution { executed_projects })
}

fn assign_contributors(
    contributors: &[Contributor],
    project: &Project,
    available_contributors_ids: &mut HashSet<usize>,
) -> Option<Vec<(usize, usize, u8)>> {
    let mut assigned_contributors = vec![];

    // TODO: sort roles (either in order of required skill level or randomly)
    for role in project.roles.iter() {
        match find_best_contributor(contributors, role, available_contributors_ids) {
            Some(contributor_data) => {
                available_contributors_ids.remove(&contributor_data.0);
                assigned_contributors.push(contributor_data);
            }
            None => return None,
        }
    }

    Some(assigned_contributors)
}

fn find_best_contributor(
    contributors: &[Contributor],
    role: &Role,
    available_contributors_ids: &HashSet<usize>,
) -> Option<(usize, usize, u8)> {
    if available_contributors_ids.is_empty() {
        return None;
    }

    let mut best_contributor_data = None;
    let mut best_contributor_loss = u8::MAX;

    for contributor_id in available_contributors_ids.iter() {
        let contributor = &contributors[*contributor_id];
        match contributor.skills.get(&role.skill_id) {
            None => {}
            Some(skill_level) => {
                // TODO: consider also contributors that can be mentored
                if *skill_level >= role.required_skill_level {
                    let loss = role.required_skill_level.wrapping_sub(*skill_level);

                    if loss < best_contributor_loss {
                        best_contributor_loss = loss;
                        best_contributor_data =
                            Some((*contributor_id, role.skill_id, role.required_skill_level));
                    }

                    if loss == 0 {
                        break;
                    }
                }
            }
        }
    }

    best_contributor_data
}

fn sort_projects<TIter: Iterator<Item = usize>>(
    input: &InputData,
    current_time: u32,
    project_ids: TIter,
) -> Vec<usize> {
    let mut sorted_result = project_ids
        .map(|project_id| {
            (
                project_id,
                score_project(current_time, &input.projects[project_id]),
            )
        })
        .collect::<Vec<_>>();

    sorted_result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    sorted_result
        .iter()
        .map(|entry| entry.0)
        .collect::<Vec<_>>()
}

fn find_best_project(
    input: &InputData,
    current_time: u32,
    available_projects_ids: &HashSet<usize>,
) -> Option<usize> {
    if available_projects_ids.is_empty() {
        return None;
    }

    let mut current_best = 0;
    let mut current_score = f32::MIN;

    for project_id in available_projects_ids.iter() {
        let score = score_project(current_time, &input.projects[*project_id]);

        if score > current_score {
            current_score = score;
            current_best = *project_id;
        }
    }

    Some(current_best)
}

fn score_project(current_time: u32, project: &Project) -> f32 {
    let extra_time = current_time as i32 + project.duration as i32 - project.best_before as i32;
    let score = if extra_time <= 0 {
        project.score as f32
    } else if extra_time as u32 >= project.duration {
        0.0
    } else {
        project.score as f32 - extra_time as f32
    };

    score / project.duration as f32 / project.roles.len() as f32
}
