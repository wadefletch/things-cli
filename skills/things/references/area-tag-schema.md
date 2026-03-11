# Area and Tag JSON Schemas

## Area Object

Returned by `things areas --json`. Returns `Area[]`.

```json
{
  "uuid": "GHI12345-1234-1234-1234-123456789GHI",
  "title": "Work",
  "tags": []
}
```

| Field | Type | Description |
|-------|------|-------------|
| `uuid` | `string` | Things internal UUID |
| `title` | `string` | Area name |
| `tags` | `string[]` | Tags associated with the area |

## Tag Object

Returned by `things tags --json`. Returns `Tag[]`.

```json
{
  "uuid": "JKL12345-1234-1234-1234-123456789JKL",
  "title": "urgent"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `uuid` | `string` | Things internal UUID |
| `title` | `string` | Tag name |
