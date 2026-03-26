pub mod human;
pub mod json;

use colored::Colorize;
use crate::models::{Area, Project, Tag, Task};
use anyhow::Result;

pub fn print_tasks(tasks: &[Task], as_json: bool) -> Result<()> {
    print_tasks_inner(tasks, as_json, false, None, false)
}

pub fn print_tasks_with_dates(tasks: &[Task], as_json: bool) -> Result<()> {
    print_tasks_inner(tasks, as_json, false, None, true)
}

pub fn print_tasks_scoped(tasks: &[Task], as_json: bool, scoped_project: &str) -> Result<()> {
    print_tasks_inner(tasks, as_json, false, Some(scoped_project), false)
}

pub fn print_tasks_with_legend(tasks: &[Task], as_json: bool, show_legend: bool) -> Result<()> {
    print_tasks_inner(tasks, as_json, show_legend, None, false)
}

fn print_tasks_inner(
    tasks: &[Task],
    as_json: bool,
    show_legend: bool,
    scoped_project: Option<&str>,
    inline_dates: bool,
) -> Result<()> {
    use crate::shortid::RefKind;

    let mut entries: Vec<(RefKind, &str)> = tasks
        .iter()
        .map(|t| (RefKind::Task, t.uuid.as_str()))
        .collect();
    let task_count = entries.len();

    let mut project_uuids: Vec<&str> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for task in tasks {
        if let Some(ref puuid) = task.project_uuid {
            if seen.insert(puuid.as_str()) {
                project_uuids.push(puuid.as_str());
            }
        }
    }
    entries.extend(project_uuids.iter().map(|u| (RefKind::Project, *u)));

    let all_refs = crate::shortid::assign(&entries);
    let task_refs = &all_refs[..task_count];

    let mut project_ref_map = std::collections::HashMap::new();
    for (i, puuid) in project_uuids.iter().enumerate() {
        if let Some(r) = all_refs.get(task_count + i) {
            project_ref_map.insert((*puuid).to_owned(), r.clone());
        }
    }

    if as_json {
        json::print_tasks(tasks)
    } else {
        human::print_tasks(tasks, show_legend, task_refs, &project_ref_map, scoped_project, inline_dates);
        Ok(())
    }
}

/// Print tasks grouped by project, mirroring the Things UI.
/// `groups` contains `(project_uuid, project_title, tasks)` tuples.
pub fn print_grouped(
    standalone: &[Task],
    groups: &[(String, String, Vec<Task>)],
    as_json: bool,
) -> Result<()> {
    use crate::shortid::RefKind;

    let mut entries: Vec<(RefKind, &str)> = standalone
        .iter()
        .map(|t| (RefKind::Task, t.uuid.as_str()))
        .collect();

    let mut project_start_indices: Vec<(usize, usize)> = Vec::new();
    for (puuid, _, tasks) in groups {
        let proj_idx = entries.len();
        entries.push((RefKind::Project, puuid.as_str()));
        let tasks_start = entries.len();
        for task in tasks {
            entries.push((RefKind::Task, task.uuid.as_str()));
        }
        project_start_indices.push((proj_idx, tasks_start));
    }

    let all_refs = crate::shortid::assign(&entries);

    if as_json {
        let groups_json: Vec<serde_json::Value> = groups
            .iter()
            .enumerate()
            .map(|(i, (_, title, tasks))| {
                let (proj_idx, _) = project_start_indices[i];
                serde_json::json!({
                    "project": title,
                    "ref": all_refs[proj_idx],
                    "tasks": tasks,
                })
            })
            .collect();
        let output = serde_json::json!({
            "tasks": standalone,
            "projects": groups_json,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    if standalone.is_empty() && groups.is_empty() {
        println!("{}", "No tasks found.".dimmed());
        return Ok(());
    }

    let standalone_refs = &all_refs[..standalone.len()];
    let project_ref_map = std::collections::HashMap::new();
    human::print_tasks(standalone, false, standalone_refs, &project_ref_map, None, false);

    for (i, (puuid, title, tasks)) in groups.iter().enumerate() {
        let (proj_idx, tasks_start) = project_start_indices[i];
        let proj_ref = &all_refs[proj_idx];
        let task_refs = &all_refs[tasks_start..tasks_start + tasks.len()];

        println!();
        human::print_project_group(title, proj_ref, puuid, tasks, task_refs);
    }

    Ok(())
}

pub fn print_task_detail(task: &Task, as_json: bool) -> Result<()> {
    if as_json {
        json::print_value(task)
    } else {
        human::print_task_detail(task);
        Ok(())
    }
}

pub fn print_projects(projects: &[Project], as_json: bool) -> Result<()> {
    if as_json {
        json::print_value(projects)
    } else {
        human::print_projects(projects);
        Ok(())
    }
}

pub fn print_areas(areas: &[Area], as_json: bool) -> Result<()> {
    if as_json {
        json::print_value(areas)
    } else {
        human::print_areas(areas);
        Ok(())
    }
}

pub fn print_tags(tags: &[Tag], as_json: bool) -> Result<()> {
    if as_json {
        json::print_value(tags)
    } else {
        human::print_tags(tags);
        Ok(())
    }
}
