pub mod human;
pub mod json;

use crate::models::{Area, Project, Tag, Task};
use anyhow::Result;

pub fn print_tasks(tasks: &[Task], as_json: bool) -> Result<()> {
    print_tasks_inner(tasks, as_json, false, None)
}

pub fn print_tasks_scoped(tasks: &[Task], as_json: bool, scoped_project: &str) -> Result<()> {
    print_tasks_inner(tasks, as_json, false, Some(scoped_project))
}

pub fn print_tasks_with_legend(tasks: &[Task], as_json: bool, show_legend: bool) -> Result<()> {
    print_tasks_inner(tasks, as_json, show_legend, None)
}

fn print_tasks_inner(
    tasks: &[Task],
    as_json: bool,
    show_legend: bool,
    scoped_project: Option<&str>,
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
        human::print_tasks(tasks, show_legend, task_refs, &project_ref_map, scoped_project);
        Ok(())
    }
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
