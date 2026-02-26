use anyhow::Result;
use rusqlite::Connection;

use crate::{db, output};

pub fn list_tags(conn: &Connection, json: bool) -> Result<()> {
    let tags = db::all_tags(conn)?;
    output::print_tags(&tags, json)
}
