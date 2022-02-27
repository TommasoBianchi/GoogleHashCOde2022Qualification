use std::collections::{HashMap, HashSet};

use crate::{
    parse_input::{Contributor, InputData, Project, Role},
    solution::{ExecutedProject, Solution},
};

use super::errors::SolveError;

pub fn solve(
    input: &InputData,
    dataset_name: String,
    max_rounds_without_improvements: u32,
) -> Result<Solution, SolveError> {
    let mut available_projects_ids: HashSet<usize> =
        (0..input.projects.len()).into_iter().collect();
    let mut available_contributors_ids: HashSet<usize> =
        (0..input.contributors.len()).into_iter().collect();
    let mut contributors_ids_to_freeup_time: HashMap<usize, u32> = HashMap::new();
    let mut current_time = 0_u32;
    let mut next_current_time = 0_u32;
    let mut rounds_without_improvements = 0_u32;
    let mut estimated_score = 0_u32;

    let mut executed_projects = vec![];

    let mut current_contributors: Vec<Contributor> = input.contributors.to_vec();

    let latest_time = input
        .projects
        .iter()
        .map(|p| p.best_before + p.score)
        .max()
        .unwrap_or(0);

    while !available_projects_ids.is_empty() && current_time < latest_time {
        // Cleanup projects no longer doable (i.e., with a score of zero)
        for project_id in available_projects_ids.iter().cloned().collect::<Vec<_>>() {
            let project = &input.projects[project_id];
            let score = score_project(current_time, project);
            if score == 0 {
                available_projects_ids.remove(&project_id);
            }
        }

        let mut round_estimated_score = 0_u32;

        // Sort projects by score
        let sorted_projects =
            sort_projects(input, current_time, available_projects_ids.iter().cloned());

        let mut assigned_projects = 0;

        for project_id in sorted_projects {
            let project = &input.projects[project_id];

            // Assign contributors to the project
            let contributors_option =
                assign_contributors(&current_contributors, project, &available_contributors_ids);

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

            // Set contributors as no longer available, store freeup time and increase skills for each contributor
            for contributor_data in contributors_ids.iter() {
                let contributor_id = contributor_data.0;
                let skill_id = contributor_data.1;
                let required_skill_level = contributor_data.2;

                available_contributors_ids.remove(&contributor_id);

                contributors_ids_to_freeup_time
                    .insert(contributor_id, current_time + project.duration);

                // Increase skill level for contributors (based on the skill they contributed for)
                let skills = &mut current_contributors[contributor_id].skills;

                match skills.get_mut(&skill_id) {
                    None => {
                        skills.insert(skill_id, 1);
                    }
                    Some(current_skill_level) => {
                        if required_skill_level >= *current_skill_level {
                            *current_skill_level += 1;
                        }
                    }
                }
            }

            assigned_projects += 1;

            let project_score = score_project(current_time, project);
            if project_score == 0 {
                println!(
                    "Current time = {}/{}, best before = {}, duration = {}, score = {}",
                    current_time, latest_time, project.best_before, project.duration, project.score
                );
            }
            round_estimated_score += project_score;

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
        }

        estimated_score += round_estimated_score;

        current_time = next_current_time.max(current_time + 1);

        if assigned_projects > 0 {
            rounds_without_improvements = 0;
        } else {
            rounds_without_improvements += 1;
        }

        println!(
            "[{}] Assigned projects = {} this round, {} total; next time = {}/{}; remaining projects = {}; estimated score = {} (+{} this round)",
            dataset_name,
            assigned_projects,
            executed_projects.len(),
            current_time,
            latest_time,
            available_projects_ids.len(),
            estimated_score,
            round_estimated_score
        );

        if rounds_without_improvements > max_rounds_without_improvements {
            println!(
                "[{}] Reached max rounds without improvements ({}), exiting.",
                dataset_name, max_rounds_without_improvements
            );
            break;
        }

        // Free up contributors
        let contributors_ids_to_free = contributors_ids_to_freeup_time
            .iter()
            .filter(|entry| *entry.1 <= current_time)
            .map(|entry| entry.0)
            .cloned()
            .collect::<Vec<_>>();
        for contributor_id in contributors_ids_to_free.iter() {
            contributors_ids_to_freeup_time.remove(contributor_id);
            available_contributors_ids.insert(*contributor_id);
        }
    }

    Ok(Solution {
        executed_projects,
        estimated_score,
    })
}

fn assign_contributors(
    contributors: &[Contributor],
    project: &Project,
    available_contributors_ids: &HashSet<usize>,
) -> Option<Vec<(usize, usize, u8)>> {
    let mut unavailable_contributors = HashSet::new();
    let mut assigned_contributors = vec![];
    let mut selected_contributors_skillset = HashMap::new();

    // TODO: sort roles (either in order of required skill level or randomly)
    for role in project.roles.iter() {
        match find_best_contributor(
            contributors,
            role,
            available_contributors_ids,
            &unavailable_contributors,
            &selected_contributors_skillset,
        ) {
            Some(contributor_data) => {
                unavailable_contributors.insert(contributor_data.0);
                assigned_contributors.push(contributor_data);

                for skill in contributors[contributor_data.0].skills.iter() {
                    match selected_contributors_skillset.get_mut(skill.0) {
                        None => {
                            selected_contributors_skillset.insert(*skill.0, *skill.1);
                        }
                        Some(v) => *v = *skill.1.max(v),
                    }
                }
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
    contributors_ids_blacklist: &HashSet<usize>,
    selected_contributors_skillset: &HashMap<usize, u8>,
) -> Option<(usize, usize, u8)> {
    if available_contributors_ids.is_empty() {
        return None;
    }

    let mut best_contributor_data = None;
    let mut best_contributor_loss = u8::MAX;

    for contributor_id in available_contributors_ids.iter() {
        if contributors_ids_blacklist.contains(contributor_id) {
            continue;
        }

        let contributor = &contributors[*contributor_id];
        let skill_level = contributor.skills.get(&role.skill_id).unwrap_or(&0);

        let can_be_mentored = selected_contributors_skillset
            .get(&role.skill_id)
            .unwrap_or(&0)
            >= &role.required_skill_level
            && *skill_level == role.required_skill_level - 1;
        if can_be_mentored || *skill_level >= role.required_skill_level {
            let loss = (skill_level + 1).wrapping_sub(role.required_skill_level);

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
                evaluate_project(current_time, &input.projects[project_id]),
            )
        })
        .collect::<Vec<_>>();

    sorted_result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    sorted_result
        .iter()
        .map(|entry| entry.0)
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
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
        let score = evaluate_project(current_time, &input.projects[*project_id]);

        if score > current_score {
            current_score = score;
            current_best = *project_id;
        }
    }

    Some(current_best)
}

fn evaluate_project(current_time: u32, project: &Project) -> f32 {
    score_project(current_time, project) as f32
        / project.duration as f32
        / project.roles.len() as f32
}

fn score_project(current_time: u32, project: &Project) -> u32 {
    let extra_time = current_time as i32 + project.duration as i32 - project.best_before as i32;

    if extra_time <= 0 {
        project.score
    } else if extra_time as u32 >= project.score {
        0
    } else {
        project.score - extra_time as u32
    }
}
