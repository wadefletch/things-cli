use anyhow::Result;

use crate::config::Config;

pub fn set_token(token: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.set_token(token.to_owned())?;
    println!("Auth token saved.");
    Ok(())
}

pub fn show_token() -> Result<()> {
    let config = Config::load()?;
    match config.masked_token() {
        Some(masked) => println!("Auth token: {masked}"),
        None => println!("No auth token set."),
    }
    Ok(())
}

pub fn clear_token() -> Result<()> {
    let mut config = Config::load()?;
    config.clear_token()?;
    println!("Auth token removed.");
    Ok(())
}
