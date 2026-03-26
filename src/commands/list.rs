use anyhow::Result;
use rusqlite::Connection;

use std::collections::BTreeMap;
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
    output::print_tasks_with_dates(&tasks, json)
}

pub fn someday(conn: &Connection, json: bool) -> Result<()> {
    let all = db::tasks_someday(conn)?;

    let mut standalone = Vec::new();
    let mut by_project: BTreeMap<String, (String, Vec<crate::models::Task>)> = BTreeMap::new();

    for task in all {
        if let (Some(ref puuid), Some(ref ptitle)) = (&task.project_uuid, &task.project_title) {
            by_project
                .entry(puuid.clone())
                .or_insert_with(|| (ptitle.clone(), Vec::new()))
                .1
                .push(task);
        } else {
            standalone.push(task);
        }
    }

    let groups: Vec<(String, String, Vec<crate::models::Task>)> = by_project
        .into_iter()
        .map(|(uuid, (title, tasks))| (uuid, title, tasks))
        .collect();

    output::print_grouped(&standalone, &groups, json)
}

pub fn logbook(conn: &Connection, since: Option<&str>, limit: usize, json: bool) -> Result<()> {
    let tasks = db::tasks_logbook(conn, since, limit)?;
    output::print_tasks_with_legend(&tasks, json, true)
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
