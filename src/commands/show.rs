use anyhow::Result;
use rusqlite::Connection;

use crate::models::TaskType;
use crate::{output, resolve};

pub fn show(conn: &Connection, id: &str, json: bool) -> Result<()> {
    let item = resolve::resolve_any(conn, id)?;
    match item.kind {
        TaskType::Project => super::projects::show_project(conn, id, json),
        _ => output::print_task_detail(&item, json),
    }
}
