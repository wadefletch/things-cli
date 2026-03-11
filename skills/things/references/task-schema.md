# Task JSON Schema

## Task Object

Returned by `things today --json`, `things inbox --json`, `things list --json`, `things search --json`, `things logbook --json`, `things upcoming --json`, `things someday --json`, and `things show <ref> --json`.

List commands return `Task[]`. The `show` command returns a single `Task`.

```json
{
  "uuid": "ABC12345-1234-1234-1234-123456789ABC",
  "title": "Buy groceries",
  "type": "task",
  "status": "open",
  "start": "started",
  "notes": "Don't forget milk",
  "project_uuid": "DEF12345-1234-1234-1234-123456789DEF",
  "project_title": "Household",
  "area_uuid": "GHI12345-1234-1234-1234-123456789GHI",
  "area_title": "Personal",
  "tags": ["errands", "quick"],
  "checklist_count": 3,
  "checklist_done": 1,
  "created_date": "2025-01-15",
  "modified_date": "2025-01-16",
  "start_date": "2025-01-17",
  "deadline": "2025-01-20",
  "completion_date": null,
  "index": 5
}
```

## Field Reference

| Field | Type | Description |
|-------|------|-------------|
| `uuid` | `string` | Things internal UUID |
| `title` | `string` | Task title |
| `type` | `enum` | `"task"`, `"project"`, or `"heading"` |
| `status` | `enum` | `"open"`, `"completed"`, or `"canceled"` |
| `start` | `enum` | `"inbox"`, `"started"`, or `"someday"` |
| `notes` | `string?` | Markdown notes, `null` if empty |
| `project_uuid` | `string?` | UUID of parent project |
| `project_title` | `string?` | Title of parent project |
| `area_uuid` | `string?` | UUID of area (direct or inherited from project) |
| `area_title` | `string?` | Title of area |
| `tags` | `string[]` | Tag names (empty array if none) |
| `checklist_count` | `int` | Total checklist items |
| `checklist_done` | `int` | Completed checklist items |
| `created_date` | `string?` | ISO date `YYYY-MM-DD` |
| `modified_date` | `string?` | ISO date `YYYY-MM-DD` |
| `start_date` | `string?` | Scheduled start date `YYYY-MM-DD` |
| `deadline` | `string?` | Deadline `YYYY-MM-DD` |
| `completion_date` | `string?` | When completed/canceled `YYYY-MM-DD` |
| `index` | `int` | Sort order within list |

## Enum Values

### `type`
| Value | Meaning |
|-------|---------|
| `"task"` | Regular to-do item (type=0 in DB) |
| `"project"` | Project container (type=1) |
| `"heading"` | Section heading within a project (type=2) |

### `status`
| Value | Meaning |
|-------|---------|
| `"open"` | Not yet completed (status=0) |
| `"completed"` | Done (status=2) |
| `"canceled"` | Canceled (status=3) |

### `start`
| Value | Meaning |
|-------|---------|
| `"inbox"` | In the Inbox (start=0) |
| `"started"` | Active/scheduled (start=1) |
| `"someday"` | Deferred to Someday (start=2) |

## Date Format

All dates are ISO `YYYY-MM-DD` strings or `null`. Internally, Things uses two date encodings:
- `created_date`, `modified_date`, `completion_date`: Unix timestamps (seconds)
- `start_date`, `deadline`: Bitpacked integers (`YYYYYYYYYYYMMMMDDDDD0000000`)

The CLI normalizes both to `YYYY-MM-DD` in output.
