// Lint configuration: extremely aggressive, opinionated clippy
#![deny(
    // Core correctness
    clippy::correctness,
    // All default warnings as errors
    clippy::all,
    // Suspicious code that is likely wrong
    clippy::suspicious,
    // Code complexity
    clippy::complexity,
    // Performance footguns
    clippy::perf,
    // Style issues
    clippy::style,
)]
#![warn(
    // ── Pedantic (the big one) ──────────────────────────────────────────
    clippy::pedantic,
    // ── Restriction lints (cherry-picked, not all) ──────────────────────
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::exit,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::infinite_loop,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::multiple_inherent_impl,
    clippy::mutex_atomic,
    clippy::needless_raw_strings,
    clippy::print_stderr,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_type_annotations,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::semicolon_inside_block,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::suspicious_xor_used_as_pow,
    clippy::todo,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::verbose_file_reads,
    // ── Nursery (experimental but useful) ───────────────────────────────
    clippy::nursery,
)]
mod cli;
mod commands;
mod config;
mod db;
mod models;
mod output;
mod resolve;
mod things_url;

use anyhow::Result;
use clap::Parser;
use cli::{AuthAction, Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    match cli.command {
        // Write commands that don't need DB
        Command::Add {
            title,
            notes,
            when_date,
            deadline,
            tags,
            list,
            heading,
            checklist,
            reveal,
        } => {
            commands::add::add(
                &title,
                notes.as_deref(),
                when_date.as_deref(),
                deadline.as_deref(),
                tags.as_deref(),
                list.as_deref(),
                heading.as_deref(),
                checklist.as_deref(),
                reveal,
            )?;
        }
        Command::Auth { action } => match action {
            AuthAction::Set { token } => commands::auth::set_token(&token)?,
            AuthAction::Show => commands::auth::show_token()?,
            AuthAction::Clear => commands::auth::clear_token()?,
        },

        // All other commands need the database
        cmd => {
            let conn = db::open()?;
            match cmd {
                Command::Today => commands::list::today(&conn, cli.json)?,
                Command::Inbox => commands::list::inbox(&conn, cli.json)?,
                Command::Upcoming => commands::list::upcoming(&conn, cli.json)?,
                Command::Someday => commands::list::someday(&conn, cli.json)?,
                Command::Logbook { since, limit } => {
                    commands::list::logbook(&conn, since.as_deref(), limit, cli.json)?;
                }
                Command::List {
                    project,
                    tag,
                    area,
                    deadline,
                } => {
                    commands::list::filtered(
                        &conn,
                        project.as_deref(),
                        tag.as_deref(),
                        area.as_deref(),
                        deadline,
                        cli.json,
                    )?;
                }
                Command::Show { id } => commands::show::show(&conn, &id, cli.json)?,
                Command::Search {
                    query,
                    include_completed,
                } => commands::search::search(&conn, &query, include_completed, cli.json)?,
                Command::Complete { id, cancel } => {
                    commands::complete::complete(&conn, &id, cancel)?;
                }
                Command::Projects { area } => {
                    commands::projects::list_projects(&conn, area.as_deref(), cli.json)?;
                }
                Command::Project { name } => {
                    commands::projects::show_project(&conn, &name, cli.json)?;
                }
                Command::Areas => commands::areas::list_areas(&conn, cli.json)?,
                Command::Tags => commands::tags::list_tags(&conn, cli.json)?,
                Command::Add { .. } | Command::Auth { .. } => {
                    // Already handled in the outer match
                }
            }
        }
    }

    Ok(())
}
