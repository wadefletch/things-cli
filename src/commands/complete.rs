use anyhow::{bail, Result};
use rusqlite::Connection;

use crate::{config::Config, resolve, things_url};

pub fn complete(conn: &Connection, id: &str, cancel: bool) -> Result<()> {
    let config = Config::load()?;
    let token = config
        .auth_token
        .as_deref()
        .filter(|t| !t.is_empty());

    let Some(token) = token else {
        bail!("Auth token required. Set one with: things auth set <token>");
    };

    let item = resolve::resolve_any(conn, id)?;

    let url = if cancel {
        things_url::cancel_task(&item.uuid, token)
    } else {
        things_url::complete_task(&item.uuid, token)
    };

    open::that(&url)?;

    if cancel {
        println!("Canceled: {}", item.title);
    } else {
        println!("Completed: {}", item.title);
    }
    Ok(())
}
