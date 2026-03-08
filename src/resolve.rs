use anyhow::{bail, Result};
use rusqlite::Connection;

use crate::db;
use crate::models::Task;
use crate::shortid::{self, RefKind};

/// Resolve any ref or UUID prefix to a task or project.
/// The ref prefix (t/p) determines the type. Bare UUID prefixes default to task.
pub fn resolve_any(conn: &Connection, identifier: &str) -> Result<Task> {
    if let Some(entry) = shortid::resolve(identifier) {
        let matches = match entry.kind {
            RefKind::Task => db::tasks_by_uuid_prefix(conn, &entry.uuid)?,
            RefKind::Project => db::projects_by_uuid_prefix(conn, &entry.uuid)?,
        };
        if matches.len() == 1 {
            return Ok(matches.into_iter().next().unwrap());
        }
    }

    // Bare UUID prefix — try task first (90% case), then project
    let task_matches = db::tasks_by_uuid_prefix(conn, identifier)?;
    if task_matches.len() == 1 {
        return Ok(task_matches.into_iter().next().unwrap());
    }
    if task_matches.len() > 1 {
        bail!(
            "Ambiguous UUID prefix '{}'. Matches:\n{}",
            identifier,
            format_matches(&task_matches)
        );
    }

    let project_matches = db::projects_by_uuid_prefix(conn, identifier)?;
    if project_matches.len() == 1 {
        return Ok(project_matches.into_iter().next().unwrap());
    }
    if project_matches.len() > 1 {
        bail!(
            "Ambiguous UUID prefix '{}'. Matches:\n{}",
            identifier,
            format_matches(&project_matches)
        );
    }

    bail!("No task or project found matching '{identifier}'")
}

/// Resolve specifically to a project.
/// Used by `things project` which always expects a project.
pub fn resolve_project(conn: &Connection, identifier: &str) -> Result<Task> {
    if let Some(entry) = shortid::resolve(identifier) {
        match entry.kind {
            RefKind::Project => {
                let matches = db::projects_by_uuid_prefix(conn, &entry.uuid)?;
                if matches.len() == 1 {
                    return Ok(matches.into_iter().next().unwrap());
                }
            }
            RefKind::Task => {
                bail!(
                    "'{identifier}' is a task ref, not a project. Use `things show {identifier}` instead."
                );
            }
        }
    }

    let uuid_matches = db::projects_by_uuid_prefix(conn, identifier)?;
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

    bail!("No project found matching '{identifier}'")
}

fn format_matches(tasks: &[Task]) -> String {
    tasks
        .iter()
        .map(|t| format!("  {} {}", &t.uuid[..8], t.title))
        .collect::<Vec<_>>()
        .join("\n")
}
