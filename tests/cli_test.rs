mod helpers;

use assert_cmd::Command;

fn things() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_things"));
    cmd.env("NO_COLOR", "1");
    cmd
}

// ── Help & version ──────────────────────────────────────────────────────

#[test]
fn help_shows_all_commands() {
    things()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("today"))
        .stdout(predicates::str::contains("inbox"))
        .stdout(predicates::str::contains("upcoming"))
        .stdout(predicates::str::contains("someday"))
        .stdout(predicates::str::contains("logbook"))
        .stdout(predicates::str::contains("list"))
        .stdout(predicates::str::contains("show"))
        .stdout(predicates::str::contains("search"))
        .stdout(predicates::str::contains("add"))
        .stdout(predicates::str::contains("complete"))
        .stdout(predicates::str::contains("projects"))
        .stdout(predicates::str::contains("project"))
        .stdout(predicates::str::contains("areas"))
        .stdout(predicates::str::contains("tags"))
        .stdout(predicates::str::contains("auth"));
}

#[test]
fn version_flag() {
    things()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains("things"));
}

#[test]
fn no_args_shows_help() {
    things()
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

// ── Subcommand help ─────────────────────────────────────────────────────

#[test]
fn add_help_shows_all_options() {
    things()
        .args(["add", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("--when"))
        .stdout(predicates::str::contains("--deadline"))
        .stdout(predicates::str::contains("--tags"))
        .stdout(predicates::str::contains("--list"))
        .stdout(predicates::str::contains("--heading"))
        .stdout(predicates::str::contains("--checklist"))
        .stdout(predicates::str::contains("--reveal"))
        .stdout(predicates::str::contains("--notes"));
}

#[test]
fn logbook_help_shows_options() {
    things()
        .args(["logbook", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("--since"))
        .stdout(predicates::str::contains("--limit"));
}

#[test]
fn list_help_shows_filters() {
    things()
        .args(["list", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("--project"))
        .stdout(predicates::str::contains("--tag"))
        .stdout(predicates::str::contains("--area"))
        .stdout(predicates::str::contains("--deadline"));
}

#[test]
fn search_help_shows_options() {
    things()
        .args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("--include-completed"));
}

#[test]
fn complete_help_shows_cancel() {
    things()
        .args(["complete", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("--cancel"));
}

// ── Global flags ────────────────────────────────────────────────────────

#[test]
fn json_flag_accepted_globally() {
    // --json shouldn't cause a parse error on any subcommand
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["--json", "today"])
        .assert()
        .success();
}

#[test]
fn no_color_flag_accepted() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["--no-color", "today"])
        .assert()
        .success();
}

#[test]
fn verbose_flag_accepted() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["-v", "today"])
        .assert()
        .success();
}

// ── Invalid input ───────────────────────────────────────────────────────

#[test]
fn unknown_subcommand_fails() {
    things()
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicates::str::contains("unrecognized subcommand"));
}

#[test]
fn add_without_title_fails() {
    things()
        .arg("add")
        .assert()
        .failure()
        .stderr(predicates::str::contains("<TITLE>"));
}

#[test]
fn show_without_id_fails() {
    things()
        .arg("show")
        .assert()
        .failure()
        .stderr(predicates::str::contains("<ID>"));
}

#[test]
fn search_without_query_fails() {
    things()
        .arg("search")
        .assert()
        .failure()
        .stderr(predicates::str::contains("<QUERY>"));
}

#[test]
fn complete_without_id_fails() {
    things()
        .arg("complete")
        .assert()
        .failure()
        .stderr(predicates::str::contains("<ID>"));
}

#[test]
fn auth_without_subcommand_fails() {
    things()
        .arg("auth")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

// ── DB not found ────────────────────────────────────────────────────────

#[test]
fn missing_db_shows_clear_error() {
    Command::new(env!("CARGO_BIN_EXE_things"))
        .env("THINGS_DB_PATH", "/nonexistent/path/main.sqlite")
        .arg("today")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Failed to open Things database"));
}
