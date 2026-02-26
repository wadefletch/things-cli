mod helpers;

use serde_json::Value;

// ── JSON output produces valid JSON ─────────────────────────────────────

#[test]
fn today_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "today"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let parsed: Value = serde_json::from_slice(&output.stdout)
        .expect("today --json should produce valid JSON");
    assert!(parsed.is_array(), "should be an array");
}

#[test]
fn today_json_contains_expected_fields() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "today"])
        .output()
        .unwrap();
    let tasks: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert!(!tasks.is_empty(), "should have tasks");

    let task = &tasks[0];
    assert!(task.get("uuid").is_some(), "should have uuid");
    assert!(task.get("title").is_some(), "should have title");
    assert!(task.get("type").is_some(), "should have type");
    assert!(task.get("status").is_some(), "should have status");
    assert!(task.get("start").is_some(), "should have start");
    assert!(task.get("tags").is_some(), "should have tags");
}

#[test]
fn today_json_uuid_is_full() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "today"])
        .output()
        .unwrap();
    let tasks: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    let uuid = tasks[0]["uuid"].as_str().unwrap();
    assert!(uuid.starts_with("task-today-"), "uuid should be full, got: {uuid}");
}

#[test]
fn today_json_tags_are_array() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "today"])
        .output()
        .unwrap();
    let tasks: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    // Find the task with tags
    let tagged = tasks.iter().find(|t| t["uuid"] == "task-today-1").unwrap();
    let tags = tagged["tags"].as_array().unwrap();
    assert!(tags.len() == 2, "should have 2 tags, got: {tags:?}");
}

#[test]
fn today_json_type_is_lowercase_string() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "today"])
        .output()
        .unwrap();
    let tasks: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(tasks[0]["type"], "task", "type should be lowercase string");
    assert_eq!(tasks[0]["status"], "open", "status should be lowercase string");
}

// ── Other commands produce valid JSON ───────────────────────────────────

#[test]
fn inbox_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "inbox"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let parsed: Value = serde_json::from_slice(&output.stdout)
        .expect("inbox --json should produce valid JSON");
    assert!(parsed.is_array());
}

#[test]
fn logbook_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "logbook"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let tasks: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert!(!tasks.is_empty(), "logbook should have completed tasks");
    // Completed tasks should have status "completed" or "canceled"
    for task in &tasks {
        let status = task["status"].as_str().unwrap();
        assert!(
            status == "completed" || status == "canceled",
            "logbook task status should be completed or canceled, got: {status}"
        );
    }
}

#[test]
fn projects_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "projects"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let projects: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert!(!projects.is_empty());
    let proj = &projects[0];
    assert!(proj.get("uuid").is_some(), "project should have uuid");
    assert!(proj.get("title").is_some(), "project should have title");
    assert!(proj.get("task_count").is_some(), "project should have task_count");
}

#[test]
fn areas_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "areas"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let areas: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(areas.len(), 2, "should have 2 areas");
}

#[test]
fn tags_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "tags"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let tags: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(tags.len(), 3, "should have 3 tags");
}

#[test]
fn search_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "search", "groceries"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let tasks: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0]["title"], "Buy groceries");
}

#[test]
fn show_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "show", "task-today-1"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let task: Value = serde_json::from_slice(&output.stdout)
        .expect("show --json should produce valid JSON");
    assert!(task.is_object(), "show should return a single object, not array");
    assert_eq!(task["title"], "Review PR #42");
}

#[test]
fn project_detail_json_is_valid() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "project", "Redesign Website"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let detail: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(detail.get("project").is_some(), "should have project key");
    assert!(detail.get("tasks").is_some(), "should have tasks key");
    assert_eq!(detail["project"]["title"], "Redesign Website");
}

// ── JSON flag position ──────────────────────────────────────────────────

#[test]
fn json_flag_works_before_subcommand() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "tags"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _: Vec<Value> = serde_json::from_slice(&output.stdout)
        .expect("--json before subcommand should work");
}

#[test]
fn json_flag_works_after_subcommand() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["tags", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _: Vec<Value> = serde_json::from_slice(&output.stdout)
        .expect("--json after subcommand should work");
}

// ── Empty results ───────────────────────────────────────────────────────

#[test]
fn empty_json_result_is_empty_array() {
    let (_dir, db_path) = helpers::create_test_db();
    let output = helpers::things_cmd(&db_path)
        .args(["--json", "search", "xyznonexistent"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let tasks: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert!(tasks.is_empty(), "no-match search should return empty array");
}

#[test]
fn empty_human_result_shows_message() {
    let (_dir, db_path) = helpers::create_test_db();
    helpers::things_cmd(&db_path)
        .args(["search", "xyznonexistent"])
        .assert()
        .success()
        .stdout(predicates::str::contains("No tasks found"));
}
