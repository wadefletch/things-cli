use urlencoding::encode;

/// Build a Things URL scheme URL for creating a task.
pub fn add_task(
    title: &str,
    notes: Option<&str>,
    when_date: Option<&str>,
    deadline: Option<&str>,
    tags: Option<&str>,
    list: Option<&str>,
    heading: Option<&str>,
    checklist: Option<&str>,
    reveal: bool,
) -> String {
    let mut params = vec![format!("title={}", encode(title))];

    if let Some(n) = notes {
        params.push(format!("notes={}", encode(n)));
    }
    if let Some(w) = when_date {
        params.push(format!("when={}", encode(w)));
    }
    if let Some(d) = deadline {
        params.push(format!("deadline={}", encode(d)));
    }
    if let Some(t) = tags {
        params.push(format!("tags={}", encode(t)));
    }
    if let Some(l) = list {
        params.push(format!("list={}", encode(l)));
    }
    if let Some(h) = heading {
        params.push(format!("heading={}", encode(h)));
    }
    if let Some(c) = checklist {
        // Things expects newline-separated checklist items
        let items = c.split(',').map(str::trim).collect::<Vec<_>>().join("\n");
        params.push(format!("checklist-items={}", encode(&items)));
    }
    if reveal {
        params.push("reveal=true".to_owned());
    }

    format!("things:///add?{}", params.join("&"))
}

/// Build a Things URL scheme URL for creating a project.
pub fn add_project(
    title: &str,
    notes: Option<&str>,
    when_date: Option<&str>,
    deadline: Option<&str>,
    tags: Option<&str>,
    area: Option<&str>,
    todos: Option<&str>,
    reveal: bool,
) -> String {
    let mut params = vec![format!("title={}", encode(title))];

    if let Some(n) = notes {
        params.push(format!("notes={}", encode(n)));
    }
    if let Some(w) = when_date {
        params.push(format!("when={}", encode(w)));
    }
    if let Some(d) = deadline {
        params.push(format!("deadline={}", encode(d)));
    }
    if let Some(t) = tags {
        params.push(format!("tags={}", encode(t)));
    }
    if let Some(a) = area {
        params.push(format!("area={}", encode(a)));
    }
    if let Some(t) = todos {
        let items = t.split(',').map(str::trim).collect::<Vec<_>>().join("\n");
        params.push(format!("to-dos={}", encode(&items)));
    }
    if reveal {
        params.push("reveal=true".to_owned());
    }

    format!("things:///add-project?{}", params.join("&"))
}

/// Build a Things URL for updating a task (requires auth token).
#[allow(clippy::too_many_arguments)]
pub fn update_task(
    id: &str,
    auth_token: &str,
    title: Option<&str>,
    notes: Option<&str>,
    when_date: Option<&str>,
    deadline: Option<&str>,
    tags: Option<&str>,
    list: Option<&str>,
    heading: Option<&str>,
    checklist_append: Option<&str>,
    checklist_prepend: Option<&str>,
    reveal: bool,
) -> String {
    let mut params = vec![
        format!("auth-token={}", encode(auth_token)),
        format!("id={}", encode(id)),
    ];

    if let Some(t) = title {
        params.push(format!("title={}", encode(t)));
    }
    if let Some(n) = notes {
        params.push(format!("notes={}", encode(n)));
    }
    if let Some(w) = when_date {
        params.push(format!("when={}", encode(w)));
    }
    if let Some(d) = deadline {
        params.push(format!("deadline={}", encode(d)));
    }
    if let Some(t) = tags {
        params.push(format!("add-tags={}", encode(t)));
    }
    if let Some(l) = list {
        params.push(format!("list={}", encode(l)));
    }
    if let Some(h) = heading {
        params.push(format!("heading={}", encode(h)));
    }
    if let Some(c) = checklist_append {
        let items = c.split(',').map(str::trim).collect::<Vec<_>>().join("\n");
        params.push(format!("append-checklist-items={}", encode(&items)));
    }
    if let Some(c) = checklist_prepend {
        let items = c.split(',').map(str::trim).collect::<Vec<_>>().join("\n");
        params.push(format!("prepend-checklist-items={}", encode(&items)));
    }
    if reveal {
        params.push("reveal=true".to_owned());
    }

    format!("things:///update?{}", params.join("&"))
}

/// Build a Things URL for completing a task (requires auth token).
pub fn complete_task(id: &str, auth_token: &str) -> String {
    format!(
        "things:///update?auth-token={}&id={}&completed=true",
        encode(auth_token),
        encode(id)
    )
}

/// Build a Things URL for canceling a task (requires auth token).
pub fn cancel_task(id: &str, auth_token: &str) -> String {
    format!(
        "things:///update?auth-token={}&id={}&canceled=true",
        encode(auth_token),
        encode(id)
    )
}
