use anyhow::{Context, Result};
use rusqlite::{params, Connection, OpenFlags};
use std::path::PathBuf;

use crate::models::{Task, TaskType, TaskStatus, StartBucket, apple_to_date_string, APPLE_EPOCH_OFFSET, Project, Area, Tag};

/// Default Things 3 database path.
fn default_db_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not determine home directory");
    home.join("Library/Group Containers/JLMPQHK86H.com.culturedcode.ThingsMac/Things Database.thingsdatabase/main.sqlite")
}

/// Open a read-only connection to the Things 3 database.
pub fn open() -> Result<Connection> {
    let path = std::env::var("THINGS_DB_PATH").map_or_else(|_| default_db_path(), PathBuf::from);

    let conn = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .with_context(|| format!("Failed to open Things database at {}", path.display()))?;

    // WAL mode is already set by Things — just ensure we can read it.
    // query_only is redundant with SQLITE_OPEN_READ_ONLY but belts-and-suspenders.
    conn.execute_batch("PRAGMA query_only=true;")?;
    Ok(conn)
}

// ---------------------------------------------------------------------------
// Query helpers
// ---------------------------------------------------------------------------

const TASK_SELECT: &str = "
    SELECT
        t.uuid,
        t.title,
        t.type AS task_type,
        t.status,
        t.start,
        t.notes,
        t.project AS project_uuid,
        p.title AS project_title,
        t.area AS area_uuid,
        COALESCE(a.title, pa.title) AS area_title,
        t.creationDate,
        t.userModificationDate,
        t.startDate,
        t.deadline,
        t.stopDate,
        t.\"index\",
        (SELECT COUNT(*) FROM TMChecklistItem ci WHERE ci.task = t.uuid) AS checklist_count,
        (SELECT COUNT(*) FROM TMChecklistItem ci WHERE ci.task = t.uuid AND ci.status = 3) AS checklist_done
    FROM TMTask t
    LEFT JOIN TMTask p ON t.project = p.uuid
    LEFT JOIN TMArea a ON t.area = a.uuid
    LEFT JOIN TMArea pa ON p.area = pa.uuid
";

fn row_to_task(row: &rusqlite::Row) -> rusqlite::Result<Task> {
    Ok(Task {
        uuid: row.get(0)?,
        title: row.get(1)?,
        kind: TaskType::from_i32(row.get(2)?),
        status: TaskStatus::from_i32(row.get(3)?),
        start: StartBucket::from_i32(row.get(4)?),
        notes: row.get(5)?,
        project_uuid: row.get(6)?,
        project_title: row.get(7)?,
        area_uuid: row.get(8)?,
        area_title: row.get(9)?,
        tags: Vec::new(), // filled in after
        checklist_count: row.get(16)?,
        checklist_done: row.get(17)?,
        created_date: apple_to_date_string(row.get(10)?),
        modified_date: apple_to_date_string(row.get(11)?),
        start_date: apple_to_date_string(row.get(12)?),
        deadline: row.get::<_, Option<i32>>(13)?.map(|d| {
            // deadline is stored as an integer YYYYMMDD-style offset or as a
            // Core Data timestamp depending on the version. In practice Things
            // stores it as a fuzzy date integer (days since 2001-01-01 are NOT
            // used here). Let's handle both.
            // Actually, the deadline column in Things stores an integer like
            // 132930000 which is a Core Data timestamp.
            // We'll try to treat small values as already-formatted and large as timestamps.
            if d > 30_000_000 {
                apple_to_date_string(Some(f64::from(d))).unwrap_or_else(|| d.to_string())
            } else {
                d.to_string()
            }
        }),
        completion_date: apple_to_date_string(row.get(14)?),
        index: row.get(15)?,
    })
}

fn fill_tags(conn: &Connection, tasks: &mut [Task]) -> Result<()> {
    if tasks.is_empty() {
        return Ok(());
    }
    let mut stmt = conn.prepare(
        "SELECT tt.tasks, tg.title
         FROM TMTaskTag tt
         JOIN TMTag tg ON tt.tags = tg.uuid
         ORDER BY tg.title",
    )?;
    let mut tag_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (task_uuid, tag_title) = row?;
        tag_map.entry(task_uuid).or_default().push(tag_title);
    }
    for task in tasks.iter_mut() {
        if let Some(tags) = tag_map.remove(&task.uuid) {
            task.tags = tags;
        }
    }
    Ok(())
}

fn query_tasks(conn: &Connection, where_clause: &str, task_params: &[&dyn rusqlite::types::ToSql]) -> Result<Vec<Task>> {
    let sql = format!("{TASK_SELECT} WHERE {where_clause} ORDER BY t.\"index\"");
    let mut stmt = conn.prepare(&sql)?;
    let mut tasks: Vec<Task> = stmt
        .query_map(task_params, row_to_task)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    fill_tags(conn, &mut tasks)?;
    Ok(tasks)
}

// ---------------------------------------------------------------------------
// Public query functions
// ---------------------------------------------------------------------------

pub fn tasks_today(conn: &Connection) -> Result<Vec<Task>> {
    query_tasks(
        conn,
        "t.type = 0 AND t.status = 0 AND t.start = 1 AND t.trashed = 0",
        &[],
    )
}

pub fn tasks_inbox(conn: &Connection) -> Result<Vec<Task>> {
    query_tasks(
        conn,
        "t.type = 0 AND t.status = 0 AND t.start = 0 AND t.trashed = 0",
        &[],
    )
}

pub fn tasks_upcoming(conn: &Connection) -> Result<Vec<Task>> {
    // Upcoming = scheduled tasks (start=1 with a startDate, or start=2 with startDate)
    // In Things, upcoming tasks have start=1 and a non-null startDate in the future
    // We'll show all tasks with a startDate that haven't been completed
    query_tasks(
        conn,
        "t.type = 0 AND t.status = 0 AND t.startDate IS NOT NULL AND t.trashed = 0",
        &[],
    )
}

pub fn tasks_someday(conn: &Connection) -> Result<Vec<Task>> {
    query_tasks(
        conn,
        "t.type = 0 AND t.status = 0 AND t.start = 2 AND t.trashed = 0",
        &[],
    )
}

pub fn tasks_logbook(conn: &Connection, since: Option<&str>, limit: usize) -> Result<Vec<Task>> {
    let base_where = "t.type = 0 AND t.status IN (2, 3) AND t.trashed = 0";

    if let Some(since_date) = since {
        let dt = chrono::NaiveDate::parse_from_str(since_date, "%Y-%m-%d")
            .with_context(|| format!("Invalid date format: {since_date}. Expected YYYY-MM-DD"))?;
        let unix_ts = dt
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();
        let apple_ts = unix_ts - APPLE_EPOCH_OFFSET;
        let limit_i64 = i64::try_from(limit).unwrap_or(i64::MAX);
        let sql = format!(
            "{TASK_SELECT} WHERE {base_where} AND t.stopDate >= ?1 ORDER BY t.stopDate DESC LIMIT ?2"
        );
        let mut stmt = conn.prepare(&sql)?;
        let mut tasks: Vec<Task> = stmt
            .query_map(params![apple_ts, limit_i64], row_to_task)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        fill_tags(conn, &mut tasks)?;
        Ok(tasks)
    } else {
        let sql = format!(
            "{TASK_SELECT} WHERE {base_where} ORDER BY t.stopDate DESC LIMIT ?1"
        );
        let mut stmt = conn.prepare(&sql)?;
        let mut tasks: Vec<Task> = stmt
            .query_map(params![i64::try_from(limit).unwrap_or(i64::MAX)], row_to_task)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        fill_tags(conn, &mut tasks)?;
        Ok(tasks)
    }
}

pub fn tasks_filtered(
    conn: &Connection,
    project: Option<&str>,
    tag: Option<&str>,
    area: Option<&str>,
    deadline_only: bool,
) -> Result<Vec<Task>> {
    let mut conditions = vec![
        "t.type = 0".to_owned(),
        "t.status = 0".to_owned(),
        "t.trashed = 0".to_owned(),
    ];
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut param_idx = 1;

    if let Some(proj) = project {
        conditions.push(format!("p.title LIKE ?{param_idx}"));
        param_values.push(Box::new(format!("%{proj}%")));
        param_idx += 1;
    }
    if let Some(area_name) = area {
        conditions.push(format!(
            "(a.title LIKE ?{param_idx} OR pa.title LIKE ?{param_idx})"
        ));
        param_values.push(Box::new(format!("%{area_name}%")));
        param_idx += 1;
    }
    if let Some(tag_name) = tag {
        conditions.push(format!(
            "t.uuid IN (SELECT tt.tasks FROM TMTaskTag tt JOIN TMTag tg ON tt.tags = tg.uuid WHERE tg.title LIKE ?{param_idx})"
        ));
        param_values.push(Box::new(format!("%{tag_name}%")));
        param_idx += 1;
    }
    let _ = param_idx;
    if deadline_only {
        conditions.push("t.deadline IS NOT NULL".to_owned());
    }

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        "{TASK_SELECT} WHERE {where_clause} ORDER BY t.\"index\""
    );

    let params_ref: Vec<&dyn rusqlite::types::ToSql> =
        param_values.iter().map(std::convert::AsRef::as_ref).collect();

    let mut stmt = conn.prepare(&sql)?;
    let mut tasks: Vec<Task> = stmt
        .query_map(params_ref.as_slice(), row_to_task)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    fill_tags(conn, &mut tasks)?;
    Ok(tasks)
}

pub fn tasks_by_uuid_prefix(conn: &Connection, prefix: &str) -> Result<Vec<Task>> {
    let pattern = format!("{prefix}%");
    query_tasks(
        conn,
        "t.uuid LIKE ?1 AND t.type = 0 AND t.trashed = 0",
        &[&pattern],
    )
}

pub fn tasks_by_title_substring(conn: &Connection, substr: &str) -> Result<Vec<Task>> {
    let pattern = format!("%{substr}%");
    query_tasks(
        conn,
        "t.title LIKE ?1 COLLATE NOCASE AND t.type = 0 AND t.status = 0 AND t.trashed = 0",
        &[&pattern],
    )
}

pub fn search_tasks(conn: &Connection, query: &str, include_completed: bool) -> Result<Vec<Task>> {
    let pattern = format!("%{query}%");
    let status_filter = if include_completed {
        "t.status IN (0, 2, 3)"
    } else {
        "t.status = 0"
    };
    query_tasks(
        conn,
        &format!(
            "(t.title LIKE ?1 COLLATE NOCASE OR t.notes LIKE ?1 COLLATE NOCASE) AND t.type = 0 AND {status_filter} AND t.trashed = 0"
        ),
        &[&pattern],
    )
}

pub fn projects_by_uuid_prefix(conn: &Connection, prefix: &str) -> Result<Vec<Task>> {
    let pattern = format!("{prefix}%");
    query_tasks(
        conn,
        "t.uuid LIKE ?1 AND t.type = 1 AND t.trashed = 0",
        &[&pattern],
    )
}

pub fn projects_by_title_substring(conn: &Connection, substr: &str) -> Result<Vec<Task>> {
    let pattern = format!("%{substr}%");
    query_tasks(
        conn,
        "t.title LIKE ?1 COLLATE NOCASE AND t.type = 1 AND t.trashed = 0",
        &[&pattern],
    )
}

pub fn all_projects(conn: &Connection, area_filter: Option<&str>) -> Result<Vec<Project>> {
    let area_join = if area_filter.is_some() {
        "AND (a.title LIKE ?1 COLLATE NOCASE)"
    } else {
        ""
    };

    let sql = format!(
        "SELECT
            t.uuid,
            t.title,
            t.status,
            t.notes,
            t.area AS area_uuid,
            a.title AS area_title,
            t.deadline,
            (SELECT COUNT(*) FROM TMTask sub WHERE sub.project = t.uuid AND sub.type = 0 AND sub.status = 0 AND sub.trashed = 0) AS task_count,
            (SELECT COUNT(*) FROM TMTask sub WHERE sub.project = t.uuid AND sub.type = 0 AND sub.status = 2 AND sub.trashed = 0) AS completed_count
        FROM TMTask t
        LEFT JOIN TMArea a ON t.area = a.uuid
        WHERE t.type = 1 AND t.trashed = 0 AND t.status = 0
        {area_join}
        ORDER BY t.\"index\""
    );

    let conn_ref = conn;
    let mut stmt = conn_ref.prepare(&sql)?;

    let params: Vec<Box<dyn rusqlite::types::ToSql>> = area_filter.map_or_else(
        Vec::new,
        |area_name| vec![Box::new(format!("%{area_name}%")) as Box<dyn rusqlite::types::ToSql>],
    );
    let params_ref: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(std::convert::AsRef::as_ref).collect();

    let projects: Vec<Project> = stmt
        .query_map(params_ref.as_slice(), |row| {
            Ok(Project {
                uuid: row.get(0)?,
                title: row.get(1)?,
                status: TaskStatus::from_i32(row.get(2)?),
                notes: row.get(3)?,
                area_uuid: row.get(4)?,
                area_title: row.get(5)?,
                tags: Vec::new(),
                deadline: row.get::<_, Option<i32>>(6)?.map(|d| {
                    if d > 30_000_000 {
                        apple_to_date_string(Some(f64::from(d))).unwrap_or_else(|| d.to_string())
                    } else {
                        d.to_string()
                    }
                }),
                task_count: row.get(7)?,
                completed_count: row.get(8)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(projects)
}

pub fn project_tasks(conn: &Connection, project_uuid: &str) -> Result<Vec<Task>> {
    query_tasks(
        conn,
        "t.project = ?1 AND t.type = 0 AND t.trashed = 0",
        &[&project_uuid],
    )
}

pub fn all_areas(conn: &Connection) -> Result<Vec<Area>> {
    let mut stmt = conn.prepare(
        "SELECT uuid, title FROM TMArea ORDER BY \"index\"",
    )?;
    let areas: Vec<Area> = stmt
        .query_map([], |row| {
            Ok(Area {
                uuid: row.get(0)?,
                title: row.get(1)?,
                tags: Vec::new(),
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(areas)
}

pub fn all_tags(conn: &Connection) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare(
        "SELECT uuid, title FROM TMTag ORDER BY title",
    )?;
    let tags: Vec<Tag> = stmt
        .query_map([], |row| {
            Ok(Tag {
                uuid: row.get(0)?,
                title: row.get(1)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(tags)
}
