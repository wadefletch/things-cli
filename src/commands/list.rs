use anyhow::Result;
use rusqlite::Connection;

use crate::{db, output};

pub fn today(conn: &Connection, json: bool) -> Result<()> {
    let tasks = db::tasks_today(conn)?;
    output::print_tasks(&tasks, json)
}

pub fn inbox(conn: &Connection, json: bool) -> Result<()> {
    let tasks = db::tasks_inbox(conn)?;
    output::print_tasks(&tasks, json)
}

pub fn upcoming(conn: &Connection, json: bool) -> Result<()> {
    let tasks = db::tasks_upcoming(conn)?;
    output::print_tasks(&tasks, json)
}

pub fn someday(conn: &Connection, json: bool) -> Result<()> {
    let tasks = db::tasks_someday(conn)?;
    output::print_tasks(&tasks, json)
}

pub fn logbook(conn: &Connection, since: Option<&str>, limit: usize, json: bool) -> Result<()> {
    let tasks = db::tasks_logbook(conn, since, limit)?;
    output::print_tasks(&tasks, json)
}

pub fn filtered(
    conn: &Connection,
    project: Option<&str>,
    tag: Option<&str>,
    area: Option<&str>,
    deadline: bool,
    json: bool,
) -> Result<()> {
    let tasks = db::tasks_filtered(conn, project, tag, area, deadline)?;
    output::print_tasks(&tasks, json)
}
