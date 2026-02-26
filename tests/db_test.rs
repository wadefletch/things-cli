mod helpers;

use predicates::prelude::*;

// ── Today ───────────────────────────────────────────────────────────────

#[test]
fn today_shows_today_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("today")
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42"))
        .stdout(predicates::str::contains("Call dentist"))
        .stdout(predicates::str::contains("Buy groceries"));
}

#[test]
fn today_excludes_inbox_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("today")
        .assert()
        .success()
        .stdout(predicates::str::contains("Sort through old photos").not())
        .stdout(predicates::str::contains("Research vacation spots").not());
}

#[test]
fn today_excludes_completed_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("today")
        .assert()
        .success()
        .stdout(predicates::str::contains("Set up CI pipeline").not());
}

#[test]
fn today_excludes_trashed_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("today")
        .assert()
        .success()
        .stdout(predicates::str::contains("Deleted task").not());
}

#[test]
fn today_excludes_headings() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("today")
        .assert()
        .success()
        .stdout(predicates::str::contains("Design Phase").not());
}

// ── Inbox ───────────────────────────────────────────────────────────────

#[test]
fn inbox_shows_inbox_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("inbox")
        .assert()
        .success()
        .stdout(predicates::str::contains("Sort through old photos"))
        .stdout(predicates::str::contains("Research vacation spots"));
}

#[test]
fn inbox_excludes_today_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("inbox")
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42").not());
}

// ── Upcoming ────────────────────────────────────────────────────────────

#[test]
fn upcoming_shows_scheduled_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("upcoming")
        .assert()
        .success()
        .stdout(predicates::str::contains("Prepare presentation"))
        .stdout(predicates::str::contains("Book hotel"));
}

// ── Someday ─────────────────────────────────────────────────────────────

#[test]
fn someday_shows_someday_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("someday")
        .assert()
        .success()
        .stdout(predicates::str::contains("Learn Rust"))
        .stdout(predicates::str::contains("Write a novel"));
}

#[test]
fn someday_excludes_today_and_inbox() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("someday")
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42").not())
        .stdout(predicates::str::contains("Sort through old photos").not());
}

// ── Logbook ─────────────────────────────────────────────────────────────

#[test]
fn logbook_shows_completed_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("logbook")
        .assert()
        .success()
        .stdout(predicates::str::contains("Set up CI pipeline"))
        .stdout(predicates::str::contains("Pack kitchen"));
}

#[test]
fn logbook_shows_canceled_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("logbook")
        .assert()
        .success()
        .stdout(predicates::str::contains("Old meeting notes"));
}

#[test]
fn logbook_excludes_open_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("logbook")
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42").not());
}

#[test]
fn logbook_limit() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["logbook", "--limit", "1"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // With limit=1, should only show one completed task
    let task_lines: Vec<&str> = stdout
        .lines()
        .filter(|l| l.contains('✓') || l.contains('✗'))
        .collect();
    assert!(task_lines.len() <= 1, "Expected at most 1 task, got {}", task_lines.len());
}

#[test]
fn logbook_since_filter() {
    let (_dir, db_path) = helpers::create_test_db();
    // stopDate 781000000 = 2025-10-02 in Apple epoch. Use a date after the earlier task.
    helpers::things_cmd(&db_path)
        .args(["logbook", "--since", "2025-10-01"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Pack kitchen"));
}

#[test]
fn logbook_since_invalid_date() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["logbook", "--since", "not-a-date"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Invalid date format"));
}

// ── List with filters ───────────────────────────────────────────────────

#[test]
fn list_filter_by_project() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["list", "--project", "Redesign Website"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42"));
}

#[test]
fn list_filter_by_area() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["list", "--area", "Personal"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Call dentist"))
        .stdout(predicates::str::contains("Buy groceries"));
}

#[test]
fn list_filter_by_tag() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["list", "--tag", "urgent"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42"))
        .stdout(predicates::str::contains("Call dentist").not());
}

#[test]
fn list_filter_by_deadline() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["list", "--deadline"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42"))
        .stdout(predicates::str::contains("Call dentist").not());
}

#[test]
fn list_no_filters_shows_all_open() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .arg("list")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should include tasks from different buckets but all open
    assert!(stdout.contains("Review PR #42") || stdout.contains("Call dentist"),
        "list should show open tasks");
    assert!(!stdout.contains("Set up CI pipeline"),
        "list should not show completed tasks");
}

// ── Show ────────────────────────────────────────────────────────────────

#[test]
fn show_by_uuid_prefix() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["show", "task-today-1"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42"))
        .stdout(predicates::str::contains("task-today-1"));
}

#[test]
fn show_by_title_substring() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["show", "Review PR"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Review PR #42"));
}

#[test]
fn show_displays_project() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["show", "Review PR #42"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Redesign Website"));
}

#[test]
fn show_displays_tags() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["show", "Review PR #42"])
        .assert()
        .success()
        .stdout(predicates::str::contains("urgent"))
        .stdout(predicates::str::contains("focus"));
}

#[test]
fn show_displays_checklist_count() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["show", "Buy groceries"])
        .assert()
        .success()
        .stdout(predicates::str::contains("1/3"));
}

#[test]
fn show_not_found() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["show", "xyznonexistent"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("No task found"));
}

#[test]
fn show_ambiguous_title() {
    let (_dir, db_path) = helpers::create_test_db();
    // "Re" matches "Review PR #42" and "Research vacation spots"
    helpers::things_cmd(&db_path)
        .args(["show", "Re"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Ambiguous"));
}

// ── Search ──────────────────────────────────────────────────────────────

#[test]
fn search_by_title() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["search", "groceries"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Buy groceries"));
}

#[test]
fn search_by_notes() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["search", "revenue"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Quarterly report"));
}

#[test]
fn search_case_insensitive() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["search", "GROCERIES"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Buy groceries"));
}

#[test]
fn search_excludes_completed_by_default() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["search", "CI pipeline"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Set up CI pipeline").not());
}

#[test]
fn search_include_completed() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["search", "CI pipeline", "--include-completed"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Set up CI pipeline"));
}

#[test]
fn search_no_results() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["search", "xyznonexistent"])
        .assert()
        .success()
        .stdout(predicates::str::contains("No tasks found"));
}

// ── Projects ────────────────────────────────────────────────────────────

#[test]
fn projects_lists_open_projects() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("projects")
        .assert()
        .success()
        .stdout(predicates::str::contains("Redesign Website"))
        .stdout(predicates::str::contains("Move Apartments"));
}

#[test]
fn projects_excludes_completed() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("projects")
        .assert()
        .success()
        .stdout(predicates::str::contains("Old Project").not());
}

#[test]
fn projects_shows_task_counts() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("projects")
        .assert()
        .success()
        .stdout(predicates::str::contains("tasks"));
}

#[test]
fn projects_filter_by_area() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["projects", "--area", "Personal"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Move Apartments"))
        .stdout(predicates::str::contains("Redesign Website").not());
}

// ── Project detail ──────────────────────────────────────────────────────

#[test]
fn project_shows_tasks() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["project", "Redesign Website"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Redesign Website"))
        .stdout(predicates::str::contains("Review PR #42"));
}

#[test]
fn project_not_found() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["project", "xyznonexistent"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("No project found"));
}

// ── Areas ───────────────────────────────────────────────────────────────

#[test]
fn areas_lists_all() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("areas")
        .assert()
        .success()
        .stdout(predicates::str::contains("Work"))
        .stdout(predicates::str::contains("Personal"));
}

// ── Tags ────────────────────────────────────────────────────────────────

#[test]
fn tags_lists_all() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .arg("tags")
        .assert()
        .success()
        .stdout(predicates::str::contains("urgent"))
        .stdout(predicates::str::contains("errand"))
        .stdout(predicates::str::contains("focus"));
}
