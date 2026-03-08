use anyhow::Result;
use rusqlite::Connection;

use crate::{db, shortid};

pub fn list_refs(conn: &Connection, clear: bool) -> Result<()> {
    if clear {
        shortid::clear();
        println!("Ref cache cleared.");
        return Ok(());
    }

    let entries = shortid::dump();
    if entries.is_empty() {
        println!("No refs cached. Run a listing command first.");
        return Ok(());
    }

    let uuids: Vec<&str> = entries.iter().map(|(_, e)| e.uuid.as_str()).collect();
    let titles = db::titles_by_uuids(conn, &uuids)?;

    for (ref_id, entry) in &entries {
        let kind = match entry.kind {
            shortid::RefKind::Task => "task",
            shortid::RefKind::Project => "project",
        };
        let title = titles.get(&entry.uuid).map_or("", String::as_str);
        let truncated = if title.len() > 60 {
            format!("{}…", &title[..title.floor_char_boundary(60)])
        } else {
            title.to_owned()
        };
        println!("{ref_id:>6}  {kind:<8}  {}  {truncated}", &entry.uuid[..8]);
    }
    Ok(())
}
