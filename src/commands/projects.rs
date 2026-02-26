use anyhow::Result;
use rusqlite::Connection;

use crate::{db, output, resolve};

pub fn list_projects(conn: &Connection, area: Option<&str>, json: bool) -> Result<()> {
    let projects = db::all_projects(conn, area)?;
    output::print_projects(&projects, json)
}

pub fn show_project(conn: &Connection, name: &str, json: bool) -> Result<()> {
    let project = resolve::resolve_project(conn, name)?;
    let tasks = db::project_tasks(conn, &project.uuid)?;

    if json {
        let detail = serde_json::json!({
            "project": {
                "uuid": project.uuid,
                "title": project.title,
                "notes": project.notes,
                "area": project.area_title,
            },
            "tasks": tasks,
        });
        println!("{}", serde_json::to_string_pretty(&detail)?);
    } else {
        use colored::Colorize;
        println!("{}", project.title.bold());
        if let Some(ref area) = project.area_title {
            println!("  Area: {area}");
        }
        if let Some(ref notes) = project.notes {
            if !notes.is_empty() {
                println!();
                println!("{notes}");
                println!();
            }
        }
        if tasks.is_empty() {
            println!("\n{}", "No tasks.".dimmed());
        } else {
            println!();
            output::print_tasks(&tasks, false)?;
        }
    }
    Ok(())
}
