use anyhow::{bail, Result};
use rusqlite::Connection;

use crate::db;
use crate::models::Task;

/// Resolve an identifier to a single task.
/// Accepts UUID prefix (like git short SHA) or case-insensitive title substring.
/// If ambiguous, returns an error listing matches.
pub fn resolve_task(conn: &Connection, identifier: &str) -> Result<Task> {
    // Try UUID prefix match first
    let uuid_matches = db::tasks_by_uuid_prefix(conn, identifier)?;
    if uuid_matches.len() == 1 {
        return Ok(uuid_matches.into_iter().next().unwrap());
    }
    if uuid_matches.len() > 1 {
        bail!(
            "Ambiguous UUID prefix '{}'. Matches:\n{}",
            identifier,
            format_matches(&uuid_matches)
        );
    }

    // Try title substring match
    let title_matches = db::tasks_by_title_substring(conn, identifier)?;
    if title_matches.len() == 1 {
        return Ok(title_matches.into_iter().next().unwrap());
    }
    if title_matches.len() > 1 {
        bail!(
            "Ambiguous title '{}'. Matches:\n{}",
            identifier,
            format_matches(&title_matches)
        );
    }

    bail!("No task found matching '{identifier}'")
}

/// Resolve an identifier to a single project.
pub fn resolve_project(conn: &Connection, identifier: &str) -> Result<Task> {
    let uuid_matches = db::projects_by_uuid_prefix(conn, identifier)?;
    if uuid_matches.len() == 1 {
        return Ok(uuid_matches.into_iter().next().unwrap());
    }
    if uuid_matches.len() > 1 {
        bail!(
            "Ambiguous UUID prefix '{}'. Matches:\n{}",
            identifier,
            format_project_matches(&uuid_matches)
        );
    }

    let title_matches = db::projects_by_title_substring(conn, identifier)?;
    if title_matches.len() == 1 {
        return Ok(title_matches.into_iter().next().unwrap());
    }
    if title_matches.len() > 1 {
        bail!(
            "Ambiguous project name '{}'. Matches:\n{}",
            identifier,
            format_project_matches(&title_matches)
        );
    }

    bail!("No project found matching '{identifier}'")
}

fn format_matches(tasks: &[Task]) -> String {
    tasks
        .iter()
        .map(|t| format!("  {} {}", &t.uuid[..8], t.title))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_project_matches(tasks: &[Task]) -> String {
    format_matches(tasks)
}
