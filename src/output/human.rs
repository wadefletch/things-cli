use std::collections::HashMap;

use colored::Colorize;

use crate::models::{TaskStatus, Task, Project, Area, Tag};

fn checkbox(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Open => "- [ ]",
        TaskStatus::Completed => "- [x]",
        TaskStatus::Canceled => "- [-]",
    }
}

fn format_ref(
    task: &Task,
    ref_id: Option<&String>,
    project_refs: &HashMap<String, String>,
    scoped_project: Option<&str>,
) -> String {
    let mut attrs: Vec<String> = Vec::new();

    if let Some(id) = ref_id {
        attrs.push(format!("ref={id}"));
    }

    if let Some(ref puuid) = task.project_uuid {
        let is_scoped = scoped_project.is_some_and(|s| s == puuid.as_str());
        if !is_scoped {
            if let Some(pref) = project_refs.get(puuid) {
                attrs.push(format!("project={pref}"));
            }
        }
    } else if let Some(ref area) = task.area_title {
        attrs.push(format!("area=\"{area}\""));
    }

    if !task.tags.is_empty() {
        let tags = task.tags.join(",");
        attrs.push(format!("tags=\"{tags}\""));
    }

    if let Some(ref d) = task.deadline {
        attrs.push(format!("deadline=\"{d}\""));
    }

    if task.checklist_count > 0 {
        attrs.push(format!("checklist=\"{}/{}\"", task.checklist_done, task.checklist_count));
    }

    if let Some(ref notes) = task.notes {
        let oneline: String = notes
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !oneline.is_empty() {
            let truncated = if oneline.len() > 80 {
                format!("{}…", &oneline[..oneline.floor_char_boundary(80)])
            } else {
                oneline
            };
            attrs.push(format!("notes=\"{truncated}\""));
        }
    }

    if attrs.is_empty() {
        String::new()
    } else {
        format!(" [{}]", attrs.join(", "))
    }
}

pub fn print_tasks(
    tasks: &[Task],
    show_legend: bool,
    short_ids: &[String],
    project_refs: &HashMap<String, String>,
    scoped_project: Option<&str>,
    inline_dates: bool,
) {
    if tasks.is_empty() {
        println!("{}", "No tasks found.".dimmed());
        return;
    }

    if show_legend && tasks.iter().any(|t| t.status == TaskStatus::Canceled) {
        println!("{}", "[x] completed  [-] canceled".dimmed());
        println!();
    }

    let mut used_project_refs: Vec<(&str, &str)> = Vec::new();
    let mut seen_projects = std::collections::HashSet::new();

    for (i, task) in tasks.iter().enumerate() {
        let cb = checkbox(task.status);
        let ref_str = format_ref(task, short_ids.get(i), project_refs, scoped_project);
        let date_prefix = if inline_dates {
            task.start_date.as_deref().map_or(String::new(), |d| format!("{d} | "))
        } else {
            String::new()
        };
        println!("{cb} {date_prefix}{}{}", task.title, ref_str.dimmed());

        if let Some(ref puuid) = task.project_uuid {
            let is_scoped = scoped_project.is_some_and(|s| s == puuid.as_str());
            if !is_scoped {
                if let Some(pref) = project_refs.get(puuid) {
                    if seen_projects.insert(puuid.as_str()) {
                        let name = task.project_title.as_deref().unwrap_or("");
                        used_project_refs.push((pref.as_str(), name));
                    }
                }
            }
        }
    }

    if !used_project_refs.is_empty() {
        println!();
        println!("{}", "Projects".dimmed());
        for (pref, name) in &used_project_refs {
            println!("  {:<6} {}", pref.dimmed(), name.dimmed());
        }
    }
}

pub fn print_project_group(
    title: &str,
    proj_ref: &str,
    proj_uuid: &str,
    tasks: &[Task],
    task_refs: &[String],
) {
    let meta = format!("[ref={proj_ref}]");
    println!("  {} {}", title.bold(), meta.dimmed());

    let project_ref_map = HashMap::new();
    for (i, task) in tasks.iter().enumerate() {
        let cb = checkbox(task.status);
        let ref_str = format_ref(task, task_refs.get(i), &project_ref_map, Some(proj_uuid));
        println!("  {cb} {}{}", task.title, ref_str.dimmed());
    }
}

pub fn print_task_detail(task: &Task) {
    let cb = checkbox(task.status);
    println!("{cb} {}", task.title.bold());
    println!("  UUID:    {}", &task.uuid);

    if let Some(ref proj) = task.project_title {
        println!("  Project: {proj}");
    }
    if let Some(ref area) = task.area_title {
        println!("  Area:    {area}");
    }
    if !task.tags.is_empty() {
        let tags = task.tags.iter().map(|t| format!("#{t}")).collect::<Vec<_>>().join(" ");
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
        let trimmed = notes.trim();
        if !trimmed.is_empty() {
            println!();
            for line in trimmed.lines() {
                println!("  {line}");
            }
        }
    }
}

pub fn print_projects(projects: &[Project]) {
    if projects.is_empty() {
        println!("{}", "No projects found.".dimmed());
        return;
    }

    for project in projects {
        let mut meta: Vec<String> = Vec::new();

        if let Some(ref area) = project.area_title {
            meta.push(area.clone());
        }

        meta.push(format!("{} tasks", project.task_count));

        if let Some(ref d) = project.deadline {
            meta.push(format!("due {d}"));
        }

        let meta_str = meta.join("  ");
        println!("- [ ] {}  {}", project.title, meta_str.dimmed());
    }
}

pub fn print_areas(areas: &[Area]) {
    if areas.is_empty() {
        println!("{}", "No areas found.".dimmed());
        return;
    }
    for area in areas {
        println!("- {}", area.title);
    }
}

pub fn print_tags(tags: &[Tag]) {
    if tags.is_empty() {
        println!("{}", "No tags found.".dimmed());
        return;
    }
    for tag in tags {
        println!("- #{}", tag.title);
    }
}
