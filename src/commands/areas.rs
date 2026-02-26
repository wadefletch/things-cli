use anyhow::Result;
use rusqlite::Connection;

use crate::{db, output};

pub fn list_areas(conn: &Connection, json: bool) -> Result<()> {
    let areas = db::all_areas(conn)?;
    output::print_areas(&areas, json)
}
