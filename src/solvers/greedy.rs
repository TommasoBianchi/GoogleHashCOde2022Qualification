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
    let mut contributors_ids_to_freeup_time: HashMap<usize, u16> = HashMap::new();
    let mut current_time = 0_u16;
    let mut executed_projects = vec![];

    let mut current_contributors: Vec<Contributor> = input.contributors.to_vec();

    let mut best_project_id_option =
        find_best_project(input, current_time, &available_projects_ids);

    while !available_projects_ids.is_empty() {
        // Find best project
        let best_project_id = best_project_id_option.unwrap();
        let best_project = &input.projects[best_project_id];

        // Assign contributors to the project
        let contributors_option = assign_contributors(
            &current_contributors,
            best_project,
            &mut available_contributors_ids,
        );

        if contributors_option.is_none() && contributors_ids_to_freeup_time.is_empty() {
            // NOTE: this is temporary
            available_projects_ids.remove(&best_project_id);
            best_project_id_option =
                find_best_project(input, current_time, &available_projects_ids);
            continue;
        }

        if contributors_option.is_none() {
            // TODO: go to second best project instead of bailing out
            current_time += 1; // TODO: optimize by advancing "enough"
                               // Free up contributors
            let contributors_ids_to_free = contributors_ids_to_freeup_time
                .iter()
                .filter(|entry| *entry.1 == current_time)
                .map(|entry| entry.0)
                .cloned()
                .collect::<Vec<_>>();
            for contributor_id in contributors_ids_to_free.iter() {
                contributors_ids_to_freeup_time.remove(contributor_id);
                available_contributors_ids.insert(*contributor_id);
            }
            continue;
        }

        let contributors_ids = contributors_option.unwrap();

        // Store freeup time and increase skills for each contributor
        for contributor_id in contributors_ids.iter() {
            contributors_ids_to_freeup_time
                .insert(*contributor_id, current_time + best_project.duration);
            // TODO: increase skills for contributors (based on the skill they contributed for)
        }

        // Save results
        executed_projects.push(ExecutedProject {
            project: best_project,
            contributors: contributors_ids
                .iter()
                .map(|contributor_id| &input.contributors[*contributor_id])
                .collect(),
        });

        // Delete best project from available ones
        available_projects_ids.remove(&best_project_id);

        // Find next best project
        best_project_id_option = find_best_project(input, current_time, &available_projects_ids);

        // Cleanup projects no longer doable
        for project_id in available_projects_ids.iter().cloned().collect::<Vec<_>>() {
            let project = &input.projects[project_id];
            if current_time >= project.best_before + project.score {
                available_projects_ids.remove(&project_id);
            }
        }

        // TODO: consider mentoring (skills level up)
    }

    Ok(Solution { executed_projects })
}

fn assign_contributors(
    contributors: &[Contributor],
    project: &Project,
    available_contributors_ids: &mut HashSet<usize>,
) -> Option<Vec<usize>> {
    let mut assigned_contributors = vec![];

    // TODO: sort roles (either in order of required skill level or randomly)
    for role in project.roles.iter() {
        match find_best_contributor(contributors, role, available_contributors_ids) {
            Some(contributor_id) => {
                available_contributors_ids.remove(&contributor_id);
                assigned_contributors.push(contributor_id);
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
) -> Option<usize> {
    if available_contributors_ids.is_empty() {
        return None;
    }

    let mut best_contributor_id = None;
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
                        best_contributor_id = Some(*contributor_id);
                    }

                    if loss == 0 {
                        break;
                    }
                }
            }
        }
    }

    best_contributor_id
}

fn find_best_project(
    input: &InputData,
    current_time: u16,
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

fn score_project(current_time: u16, project: &Project) -> f32 {
    let extra_time = current_time as i32 + project.duration as i32 - project.best_before as i32;
    let score = if extra_time <= 0 {
        project.score as f32
    } else if extra_time as u16 >= project.duration {
        0.0
    } else {
        project.score as f32 - extra_time as f32
    };

    score / project.duration as f32 / project.roles.len() as f32
}
