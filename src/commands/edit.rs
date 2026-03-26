use anyhow::{bail, Result};
use rusqlite::Connection;

use crate::{config::Config, resolve, things_url};

#[allow(clippy::too_many_arguments)]
pub fn edit(
    conn: &Connection,
    id: &str,
    title: Option<&str>,
    notes: Option<&str>,
    when_date: Option<&str>,
    deadline: Option<&str>,
    tags: Option<&str>,
    list: Option<&str>,
    heading: Option<&str>,
    checklist_append: Option<&str>,
    checklist_prepend: Option<&str>,
    reveal: bool,
) -> Result<()> {
    let config = Config::load()?;
    let token = config
        .auth_token
        .as_deref()
        .filter(|t| !t.is_empty());

    let Some(token) = token else {
        bail!("Auth token required. Set one with: things auth set <token>");
    };

    let item = resolve::resolve_any(conn, id)?;

    let url = things_url::update_task(
        &item.uuid, token, title, notes, when_date, deadline, tags, list, heading,
        checklist_append, checklist_prepend, reveal,
    );

    open::that(&url)?;
    println!("Updated: {}", item.title);
    Ok(())
}
