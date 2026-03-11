---
name: things
description: >
  Manage Things 3 tasks and projects on macOS via the `things` CLI. Use this skill whenever the user
  mentions tasks, to-dos, todos, inbox, today list, projects, areas, tags, deadlines, someday,
  upcoming, logbook, task management, Things 3, or wants to add/edit/complete/search tasks. Also use
  when reviewing what's on the user's plate, doing a daily review, checking project status, or
  managing any personal task workflow.
allowed-tools:
  - Bash(things *)
---

# Things 3 CLI

`things` reads from the Things 3 SQLite database (read-only) and uses the Things URL scheme for writes. macOS only.

## Core Workflow

1. **List tasks** to get refs: `things today --json`
2. **Use refs** to inspect or act: `things show t1 --json`, `things complete t1`
3. **Re-list** after mutations to get fresh refs

```bash
things today --json
things show t1 --json
things complete t1
things today --json
```

Always pass `--json` for structured, parseable output.

## Essential Commands

```bash
things today --json
things inbox --json
things upcoming --json
things someday --json
things logbook --json
things logbook --since 2025-01-01 --limit 20 --json

things list --project "Website" --json
things list --tag "urgent" --json
things list --area "Work" --deadline --json

things show t1 --json
things search "groceries" --json
things search "meeting" --include-completed --json

things projects --json
things projects --area "Work" --json
things project p1 --json
things project "Website Redesign" --json

things areas --json
things tags --json

things add "Buy milk"
things add "Write report" --notes "Include Q4 data" --when today --deadline 2025-03-15
things add "Design review" --list "Website Redesign" --tags "design,review"
things add "Pack list" --checklist "Passport,Charger,Clothes"

things edit t1 --title "Updated title" --when tomorrow
things edit t1 --deadline 2025-04-01 --tags "priority,q2"

things complete t1
things complete p2 --cancel
```

## Ref System

List commands assign short refs to each result: `t1`, `t2` for tasks, `p1`, `p2` for projects. Refs persist across commands until explicitly cleared.

- Refs are assigned when you run a list command (`today`, `inbox`, `list`, `search`, `projects`, etc.)
- Use refs anywhere an `<id>` is expected: `things show t1`, `things edit t1`, `things complete t1`
- Refs also accept `@t1` or `ref=t1` syntax
- UUID prefixes work too: `things show ABC123`
- Clear refs with `things refs --clear`
- View current refs with `things refs`

**Lifecycle**: Refs accumulate across commands. The same UUID always gets the same ref. Run `things refs --clear` to reset if the numbers get high.

## Common Patterns

### Daily Review
```bash
things today --json
things inbox --json
```

### Create a Task for Today
```bash
things add "Review PR #42" --when today --list "Engineering" --tags "code-review"
```

### Move Inbox Tasks
```bash
things inbox --json
things edit t1 --when today
things edit t2 --when someday
things edit t3 --list "Q2 Planning"
```

### Project Review
```bash
things projects --json
things project p1 --json
```

### Check What Got Done
```bash
things logbook --since 2025-03-01 --json
```

### Search and Act
```bash
things search "deploy" --json
things complete t1
```

## Auth Setup

`edit` and `complete` require a Things auth token.

1. Open Things > Settings > General > Enable Things URLs
2. Copy the auth token
3. Run: `things auth set "<token>"`

Check token status: `things auth show`

## Important Notes

- **macOS only** — reads the Things 3 SQLite database directly
- **Writes open Things.app** — `add`, `edit`, `complete` use the `things:///` URL scheme, which activates Things
- **Always use `--json`** — human output is for terminal display; JSON is for programmatic use
- **Read operations are instant** — they query the local SQLite database
- **Filters use substring matching** — `--project "Web"` matches "Website Redesign"
- **`--when` accepts keywords** — `today`, `tomorrow`, `evening`, `someday`, or `YYYY-MM-DD`
- **Tags and checklist are comma-separated** — `--tags "a,b,c"`, `--checklist "item1,item2"`

## Deep-Dive Documentation

| Reference | When to Use |
|-----------|-------------|
| [references/commands.md](references/commands.md) | Full command reference with all flags and examples |
| [references/task-schema.md](references/task-schema.md) | Task JSON shape, all fields, enum values |
| [references/project-schema.md](references/project-schema.md) | Project list and detail JSON shapes |
| [references/area-tag-schema.md](references/area-tag-schema.md) | Area and Tag JSON shapes |
