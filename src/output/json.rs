use anyhow::Result;
use serde::Serialize;

pub fn print_tasks(tasks: &[crate::models::Task]) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    println!("{json}");
    Ok(())
}

pub fn print_value<T: Serialize + ?Sized>(value: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
