use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn things_with_home(home: &std::path::Path) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_things"));
    cmd.env("HOME", home);
    cmd.env("NO_COLOR", "1");
    cmd
}

#[test]
fn auth_set_then_show() {
    let home = TempDir::new().unwrap();
    things_with_home(home.path())
        .args(["auth", "set", "my-secret-token"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Auth token saved"));

    things_with_home(home.path())
        .args(["auth", "show"])
        .assert()
        .success()
        .stdout(predicates::str::contains("my-s****"))
        .stdout(predicates::str::contains("my-secret-token").not());
}

#[test]
fn auth_show_when_no_token() {
    let home = TempDir::new().unwrap();
    things_with_home(home.path())
        .args(["auth", "show"])
        .assert()
        .success()
        .stdout(predicates::str::contains("No auth token set"));
}

#[test]
fn auth_set_then_clear_then_show() {
    let home = TempDir::new().unwrap();
    things_with_home(home.path())
        .args(["auth", "set", "token123"])
        .assert()
        .success();

    things_with_home(home.path())
        .args(["auth", "clear"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Auth token removed"));

    things_with_home(home.path())
        .args(["auth", "show"])
        .assert()
        .success()
        .stdout(predicates::str::contains("No auth token set"));
}

#[test]
fn auth_config_file_has_restricted_permissions() {
    let home = TempDir::new().unwrap();
    things_with_home(home.path())
        .args(["auth", "set", "secret"])
        .assert()
        .success();

    let config_path = home
        .path()
        .join("Library/Application Support/things-cli/config.toml");
    assert!(config_path.exists(), "config file should exist at {}", config_path.display());

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::metadata(&config_path).unwrap().permissions();
        let mode = perms.mode() & 0o777;
        assert_eq!(mode, 0o600, "config should have 0600 permissions, got: {mode:o}");
    }
}

#[test]
fn auth_set_overwrites_previous() {
    let home = TempDir::new().unwrap();
    things_with_home(home.path())
        .args(["auth", "set", "first-token"])
        .assert()
        .success();

    things_with_home(home.path())
        .args(["auth", "set", "second-token"])
        .assert()
        .success();

    things_with_home(home.path())
        .args(["auth", "show"])
        .assert()
        .success()
        .stdout(predicates::str::contains("seco****"));
}

#[test]
fn auth_short_token_fully_masked() {
    let home = TempDir::new().unwrap();
    things_with_home(home.path())
        .args(["auth", "set", "ab"])
        .assert()
        .success();

    things_with_home(home.path())
        .args(["auth", "show"])
        .assert()
        .success()
        .stdout(predicates::str::contains("****"))
        .stdout(predicates::str::contains("ab").not());
}

#[test]
fn complete_without_auth_token_fails() {
    let home = TempDir::new().unwrap();
    // Create a test DB so the DB opens, but no auth token set
    let (_db_dir, db_path) = {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("main.sqlite");
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch(
            "
            CREATE TABLE TMArea (uuid TEXT PRIMARY KEY, title TEXT, \"index\" INTEGER DEFAULT 0);
            CREATE TABLE TMTag (uuid TEXT PRIMARY KEY, title TEXT);
            CREATE TABLE TMTask (uuid TEXT PRIMARY KEY, title TEXT, type INTEGER DEFAULT 0, status INTEGER DEFAULT 0, start INTEGER DEFAULT 0, notes TEXT, project TEXT, area TEXT, creationDate REAL, userModificationDate REAL, startDate REAL, deadline INTEGER, stopDate REAL, trashed INTEGER DEFAULT 0, \"index\" INTEGER DEFAULT 0);
            CREATE TABLE TMTaskTag (tasks TEXT, tags TEXT);
            CREATE TABLE TMChecklistItem (uuid TEXT PRIMARY KEY, task TEXT, title TEXT, status INTEGER DEFAULT 0);
            INSERT INTO TMTask (uuid, title, type, status, start, trashed) VALUES ('t1', 'Test task', 0, 0, 1, 0);
            ",
        )
        .unwrap();
        (dir, path)
    };

    Command::new(env!("CARGO_BIN_EXE_things"))
        .env("HOME", home.path())
        .env("THINGS_DB_PATH", &db_path)
        .args(["complete", "Test task"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Auth token required"));
}
