# Project JSON Schema

## Project List Object

Returned by `things projects --json`. Returns `Project[]`.

```json
{
  "uuid": "DEF12345-1234-1234-1234-123456789DEF",
  "title": "Website Redesign",
  "status": "open",
  "notes": "Q2 project for marketing site",
  "area_uuid": "GHI12345-1234-1234-1234-123456789GHI",
  "area_title": "Work",
  "tags": ["design"],
  "task_count": 8,
  "completed_count": 3,
  "deadline": "2025-03-31"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `uuid` | `string` | Things internal UUID |
| `title` | `string` | Project title |
| `status` | `enum` | `"open"`, `"completed"`, or `"canceled"` |
| `notes` | `string?` | Project notes |
| `area_uuid` | `string?` | UUID of parent area |
| `area_title` | `string?` | Title of parent area |
| `tags` | `string[]` | Tag names |
| `task_count` | `int` | Open tasks in project |
| `completed_count` | `int` | Completed tasks in project |
| `deadline` | `string?` | Deadline `YYYY-MM-DD` |

## Project Detail Object

Returned by `things project <name> --json`. Wraps the project metadata and its tasks.

```json
{
  "project": {
    "uuid": "DEF12345-1234-1234-1234-123456789DEF",
    "title": "Website Redesign",
    "notes": "Q2 project for marketing site",
    "area": "Work"
  },
  "tasks": [
    {
      "uuid": "...",
      "title": "Create wireframes",
      "type": "task",
      "status": "open",
      ...
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `project.uuid` | `string` | Project UUID |
| `project.title` | `string` | Project title |
| `project.notes` | `string?` | Project notes |
| `project.area` | `string?` | Area title |
| `tasks` | `Task[]` | All tasks in the project (see [task-schema.md](task-schema.md)) |

The `project` field in the detail response is a simplified shape (just `uuid`, `title`, `notes`, `area`), not the full `Project` list object. The `tasks` array contains full `Task` objects.
