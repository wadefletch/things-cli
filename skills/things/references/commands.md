# Command Reference

All commands support `--json` (global flag) for structured output.

## Read Commands

### `things today`
Show tasks scheduled for today.
```bash
things today --json
```

### `things inbox`
Show inbox tasks (unscheduled, uncategorized).
```bash
things inbox --json
```

### `things upcoming`
Show upcoming scheduled tasks.
```bash
things upcoming --json
```

### `things someday`
Show tasks deferred to Someday.
```bash
things someday --json
```

### `things logbook`
Show completed and canceled tasks.
```bash
things logbook --json
things logbook --since 2025-01-01 --json
things logbook --limit 20 --json
```
| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--since` | `YYYY-MM-DD` | none | Only tasks completed since this date |
| `--limit` | `int` | `50` | Maximum number of tasks |

### `things list`
List tasks with filters. Multiple filters are AND-ed.
```bash
things list --project "Website" --json
things list --tag "urgent" --json
things list --area "Work" --json
things list --deadline --json
things list --project "Website" --tag "design" --json
```
| Flag | Type | Description |
|------|------|-------------|
| `--project` | `string` | Filter by project name (substring match) |
| `--tag` | `string` | Filter by tag name (substring match) |
| `--area` | `string` | Filter by area name (substring match) |
| `--deadline` | flag | Only show tasks with deadlines |

### `things show <id>`
Show details for a task or project by ref or UUID prefix.
```bash
things show t1 --json
things show p2 --json
things show ABC123 --json
```

### `things search <query>`
Search tasks by title or notes (case-insensitive substring match).
```bash
things search "groceries" --json
things search "meeting" --include-completed --json
```
| Flag | Type | Description |
|------|------|-------------|
| `--include-completed` | flag | Include completed tasks in results |

### `things projects`
List all open projects.
```bash
things projects --json
things projects --area "Work" --json
```
| Flag | Type | Description |
|------|------|-------------|
| `--area` | `string` | Filter by area name (substring match) |

### `things project <name>`
Show a project and its tasks. Accepts project ref, UUID prefix, or name.
```bash
things project p1 --json
things project "Website Redesign" --json
```

### `things areas`
List all areas.
```bash
things areas --json
```

### `things tags`
List all tags.
```bash
things tags --json
```

### `things refs`
Show or clear the ref cache.
```bash
things refs
things refs --clear
```
| Flag | Description |
|------|-------------|
| `--clear` | Clear all cached refs |

## Write Commands

Write commands use the Things URL scheme (`things:///`), which opens Things.app.

### `things add <title>`
Add a new task.
```bash
things add "Buy milk"
things add "Write report" --notes "Include Q4 data" --when today --deadline 2025-03-15
things add "Design review" --list "Website Redesign" --tags "design,review"
things add "Pack list" --checklist "Passport,Charger,Clothes" --when tomorrow
```
| Flag | Type | Description |
|------|------|-------------|
| `--notes` | `string` | Task notes |
| `--when` | `string` | `today`, `tomorrow`, `evening`, `someday`, or `YYYY-MM-DD` |
| `--deadline` | `YYYY-MM-DD` | Deadline date |
| `--tags` | `string` | Comma-separated tag names |
| `--list` | `string` | Project or area to add to |
| `--heading` | `string` | Heading within project |
| `--checklist` | `string` | Comma-separated checklist items |
| `--reveal` | flag | Reveal task in Things after creating |

### `things edit <id>`
Edit a task or project. Requires auth token.
```bash
things edit t1 --title "Updated title"
things edit t1 --when tomorrow --deadline 2025-04-01
things edit p2 --notes "New project notes" --tags "priority,q2"
```
| Flag | Type | Description |
|------|------|-------------|
| `--title` | `string` | New title |
| `--notes` | `string` | New notes (replaces existing) |
| `--when` | `string` | `today`, `tomorrow`, `evening`, `someday`, or `YYYY-MM-DD` |
| `--deadline` | `YYYY-MM-DD` | New deadline |
| `--tags` | `string` | Comma-separated tags (replaces existing) |
| `--list` | `string` | Move to project or area |
| `--heading` | `string` | Heading within project |
| `--reveal` | flag | Reveal in Things after editing |

### `things complete <id>`
Complete or cancel a task or project. Requires auth token.
```bash
things complete t1
things complete p2 --cancel
```
| Flag | Description |
|------|-------------|
| `--cancel` | Cancel instead of completing |

## Auth Commands

### `things auth set <token>`
Store the auth token for write operations (edit, complete).
```bash
things auth set "your-token-here"
```
Get the token from Things > Settings > General > Enable Things URLs > Auth Token.

### `things auth show`
Show the stored auth token (masked).

### `things auth clear`
Remove the stored auth token.

## Global Flags

| Flag | Description |
|------|-------------|
| `--json` | Output as JSON (structured, parseable) |
| `--no-color` | Disable colored terminal output |
