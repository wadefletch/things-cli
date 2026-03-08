use anyhow::Result;
use rusqlite::Connection;

use crate::{db, output};

pub fn search(conn: &Connection, query: &str, include_completed: bool, json: bool) -> Result<()> {
    let tasks = db::search_tasks(conn, query, include_completed)?;
    output::print_tasks_with_legend(&tasks, json, include_completed)
}
