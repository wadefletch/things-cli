use anyhow::Result;
use rusqlite::Connection;

use crate::{output, resolve};

pub fn show(conn: &Connection, id: &str, json: bool) -> Result<()> {
    let task = resolve::resolve_task(conn, id)?;
    output::print_task_detail(&task, json)
}
