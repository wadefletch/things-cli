use colored::Colorize;
use comfy_table::{presets::NOTHING, Cell, Table};

use crate::models::{TaskStatus, Task, Project, Area, Tag};

const fn status_icon(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Open => "○",
        TaskStatus::Completed => "✓",
        TaskStatus::Canceled => "✗",
    }
}

fn context_label(task: &Task) -> String {
    task.project_title
        .as_deref()
        .or(task.area_title.as_deref())
        .unwrap_or("").to_owned()
}

pub fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("{}", "No tasks found.".dimmed());
        return;
    }

    let mut table = Table::new();
    table.load_preset(NOTHING);

    for task in tasks {
        let icon = match task.status {
            TaskStatus::Open => status_icon(task.status).blue().to_string(),
            TaskStatus::Completed => status_icon(task.status).green().to_string(),
            TaskStatus::Canceled => status_icon(task.status).red().to_string(),
        };

        let ctx = context_label(task);
        let deadline_str = task
            .deadline
            .as_ref()
            .map(|d| format!("due: {d}"))
            .unwrap_or_default();

        let tags_str = if task.tags.is_empty() {
            String::new()
        } else {
            task.tags.iter().map(|t| format!("#{t}")).collect::<Vec<_>>().join(" ")
        };

        let meta = [ctx, tags_str, deadline_str]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("  ");

        table.add_row(vec![
            Cell::new(&icon),
            Cell::new(&task.title),
            Cell::new(meta.dimmed().to_string()),
        ]);
    }

    println!("{table}");
}

pub fn print_task_detail(task: &Task) {
    let icon = status_icon(task.status);
    println!("{icon} {}", task.title.bold());
    println!("  UUID:    {}", &task.uuid);
    println!("  Status:  {:?}", task.status);

    if let Some(ref proj) = task.project_title {
        println!("  Project: {proj}");
    }
    if let Some(ref area) = task.area_title {
        println!("  Area:    {area}");
    }
    if !task.tags.is_empty() {
        let tags = task.tags.join(", ");
        println!("  Tags:    {tags}");
    }
    if let Some(ref d) = task.start_date {
        println!("  When:    {d}");
    }
    if let Some(ref d) = task.deadline {
        println!("  Due:     {d}");
    }
    if let Some(ref d) = task.created_date {
        println!("  Created: {d}");
    }
    if let Some(ref d) = task.completion_date {
        println!("  Done:    {d}");
    }
    if task.checklist_count > 0 {
        println!(
            "  Checklist: {}/{}",
            task.checklist_done, task.checklist_count
        );
    }
    if let Some(ref notes) = task.notes {
        if !notes.is_empty() {
            println!();
            println!("{notes}");
        }
    }
}

pub fn print_projects(projects: &[Project]) {
    if projects.is_empty() {
        println!("{}", "No projects found.".dimmed());
        return;
    }

    let mut table = Table::new();
    table.load_preset(NOTHING);

    for project in projects {
        let area = project.area_title.as_deref().unwrap_or("");
        let counts = format!("{} tasks", project.task_count);
        let deadline_str = project
            .deadline
            .as_ref()
            .map(|d| format!("due: {d}"))
            .unwrap_or_default();

        let meta = [area.to_owned(), deadline_str]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("  ");

        table.add_row(vec![
            Cell::new(&project.title),
            Cell::new(counts.dimmed().to_string()),
            Cell::new(meta.dimmed().to_string()),
        ]);
    }

    println!("{table}");
}

pub fn print_areas(areas: &[Area]) {
    if areas.is_empty() {
        println!("{}", "No areas found.".dimmed());
        return;
    }
    for area in areas {
        println!("{}", area.title);
    }
}

pub fn print_tags(tags: &[Tag]) {
    if tags.is_empty() {
        println!("{}", "No tags found.".dimmed());
        return;
    }
    for tag in tags {
        println!("{}", tag.title);
    }
}
