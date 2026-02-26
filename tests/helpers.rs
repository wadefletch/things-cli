use assert_cmd::Command;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Path to the compiled `things` binary. Set automatically by cargo during integration tests.
fn things_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_things"))
}

/// Create a minimal Things 3-compatible SQLite database in a temp directory.
/// Returns (TempDir, path to the .sqlite file).
pub fn create_test_db() -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();
    let db_path = dir.path().join("main.sqlite");
    populate_db(&db_path);
    (dir, db_path)
}

fn populate_db(path: &Path) {
    let conn = rusqlite::Connection::open(path).unwrap();

    conn.execute_batch(
        "
        CREATE TABLE TMArea (
            uuid TEXT PRIMARY KEY,
            title TEXT,
            \"index\" INTEGER DEFAULT 0
        );

        CREATE TABLE TMTag (
            uuid TEXT PRIMARY KEY,
            title TEXT
        );

        CREATE TABLE TMTask (
            uuid TEXT PRIMARY KEY,
            title TEXT,
            type INTEGER DEFAULT 0,
            status INTEGER DEFAULT 0,
            start INTEGER DEFAULT 0,
            notes TEXT,
            project TEXT,
            area TEXT,
            creationDate REAL,
            userModificationDate REAL,
            startDate REAL,
            deadline INTEGER,
            stopDate REAL,
            trashed INTEGER DEFAULT 0,
            \"index\" INTEGER DEFAULT 0,
            FOREIGN KEY (project) REFERENCES TMTask(uuid),
            FOREIGN KEY (area) REFERENCES TMArea(uuid)
        );

        CREATE TABLE TMTaskTag (
            tasks TEXT,
            tags TEXT,
            FOREIGN KEY (tasks) REFERENCES TMTask(uuid),
            FOREIGN KEY (tags) REFERENCES TMTag(uuid)
        );

        CREATE TABLE TMChecklistItem (
            uuid TEXT PRIMARY KEY,
            task TEXT,
            title TEXT,
            status INTEGER DEFAULT 0,
            FOREIGN KEY (task) REFERENCES TMTask(uuid)
        );

        -- Areas
        INSERT INTO TMArea (uuid, title, \"index\") VALUES
            ('area-work-uuid', 'Work', 0),
            ('area-personal-uuid', 'Personal', 1);

        -- Tags
        INSERT INTO TMTag (uuid, title) VALUES
            ('tag-urgent-uuid', 'urgent'),
            ('tag-errand-uuid', 'errand'),
            ('tag-focus-uuid', 'focus');

        -- Projects (type=1)
        INSERT INTO TMTask (uuid, title, type, status, start, area, trashed, \"index\") VALUES
            ('proj-website-uuid', 'Redesign Website', 1, 0, 1, 'area-work-uuid', 0, 0),
            ('proj-move-uuid', 'Move Apartments', 1, 0, 1, 'area-personal-uuid', 0, 1),
            ('proj-archived-uuid', 'Old Project', 1, 2, 1, 'area-work-uuid', 0, 2);

        -- Inbox tasks (start=0)
        INSERT INTO TMTask (uuid, title, type, status, start, notes, trashed, \"index\") VALUES
            ('task-inbox-1', 'Sort through old photos', 0, 0, 0, NULL, 0, 0),
            ('task-inbox-2', 'Research vacation spots', 0, 0, 0, 'Check flights to Japan', 0, 1);

        -- Today tasks (start=1, status=0)
        INSERT INTO TMTask (uuid, title, type, status, start, project, area, trashed, \"index\", deadline) VALUES
            ('task-today-1', 'Review PR #42', 0, 0, 1, 'proj-website-uuid', NULL, 0, 0, 789000000),
            ('task-today-2', 'Call dentist', 0, 0, 1, NULL, 'area-personal-uuid', 0, 1, NULL),
            ('task-today-3', 'Buy groceries', 0, 0, 1, NULL, 'area-personal-uuid', 0, 2, NULL);

        -- Upcoming tasks (startDate set)
        INSERT INTO TMTask (uuid, title, type, status, start, startDate, trashed, \"index\", area) VALUES
            ('task-upcoming-1', 'Prepare presentation', 0, 0, 1, 760000000.0, 0, 0, 'area-work-uuid'),
            ('task-upcoming-2', 'Book hotel', 0, 0, 1, 761000000.0, 0, 1, 'area-personal-uuid');

        -- Someday tasks (start=2)
        INSERT INTO TMTask (uuid, title, type, status, start, trashed, \"index\") VALUES
            ('task-someday-1', 'Learn Rust', 0, 0, 2, 0, 0),
            ('task-someday-2', 'Write a novel', 0, 0, 2, 0, 1);

        -- Completed tasks (status=2) with stopDate
        INSERT INTO TMTask (uuid, title, type, status, start, stopDate, trashed, \"index\", project) VALUES
            ('task-done-1', 'Set up CI pipeline', 0, 2, 1, 780000000.0, 0, 0, 'proj-website-uuid'),
            ('task-done-2', 'Pack kitchen', 0, 2, 1, 781000000.0, 0, 1, 'proj-move-uuid');

        -- Canceled task (status=3)
        INSERT INTO TMTask (uuid, title, type, status, start, stopDate, trashed, \"index\") VALUES
            ('task-canceled-1', 'Old meeting notes', 0, 3, 1, 770000000.0, 0, 0);

        -- Trashed task (should never appear)
        INSERT INTO TMTask (uuid, title, type, status, start, trashed, \"index\") VALUES
            ('task-trashed-1', 'Deleted task', 0, 0, 1, 1, 0);

        -- Task with notes for search
        INSERT INTO TMTask (uuid, title, type, status, start, notes, trashed, \"index\", area) VALUES
            ('task-notes-1', 'Quarterly report', 0, 0, 1, 'Include revenue numbers and growth metrics', 0, 5, 'area-work-uuid');

        -- Headings (type=2, should not appear in task lists)
        INSERT INTO TMTask (uuid, title, type, status, start, project, trashed, \"index\") VALUES
            ('heading-1', 'Design Phase', 2, 0, 1, 'proj-website-uuid', 0, 0);

        -- Tag associations
        INSERT INTO TMTaskTag (tasks, tags) VALUES
            ('task-today-1', 'tag-urgent-uuid'),
            ('task-today-1', 'tag-focus-uuid'),
            ('task-today-3', 'tag-errand-uuid'),
            ('task-inbox-2', 'tag-errand-uuid');

        -- Checklist items
        INSERT INTO TMChecklistItem (uuid, task, title, status) VALUES
            ('cl-1', 'task-today-3', 'Milk', 0),
            ('cl-2', 'task-today-3', 'Eggs', 3),
            ('cl-3', 'task-today-3', 'Bread', 0);
        ",
    )
    .unwrap();
}

/// Build a `things` command pointing at the given test database.
pub fn things_cmd(db_path: &Path) -> Command {
    let mut cmd = Command::new(things_bin());
    cmd.env("THINGS_DB_PATH", db_path);
    cmd.env("NO_COLOR", "1");
    cmd
}
