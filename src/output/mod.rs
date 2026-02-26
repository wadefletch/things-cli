pub mod human;
pub mod json;

use crate::models::{Area, Project, Tag, Task};
use anyhow::Result;

pub fn print_tasks(tasks: &[Task], as_json: bool) -> Result<()> {
    if as_json {
        json::print_tasks(tasks)
    } else {
        human::print_tasks(tasks);
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
