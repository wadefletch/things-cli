use anyhow::Result;

use crate::things_url;

pub fn add(
    title: &str,
    notes: Option<&str>,
    when_date: Option<&str>,
    deadline: Option<&str>,
    tags: Option<&str>,
    list: Option<&str>,
    heading: Option<&str>,
    checklist: Option<&str>,
    reveal: bool,
) -> Result<()> {
    let url = things_url::add_task(title, notes, when_date, deadline, tags, list, heading, checklist, reveal);
    open::that(&url)?;
    println!("Created task: {title}");
    Ok(())
}
