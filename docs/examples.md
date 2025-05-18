# Usage Examples

This section provide a how-to use the endpoints with examples.

## Companies

### List Companies

```bash
curl http://localhost:3000/companies
```

### Filter Companies by Name

```bash
curl http://localhost:3000/companies?name=tech
```

### Create a Company

```bash
curl -X POST http://localhost:3000/companies \
  -H "Content-Type: application/json" \
  -d '{"name": "Acme Inc."}'
```

### Get a Company by ID

```bash
curl http://localhost:3000/companies/YOUR_COMPANY_ID
```

### Update a Company

```bash
curl -X PUT http://localhost:3000/companies/YOUR_COMPANY_ID \
  -H "Content-Type: application/json" \
  -d '{"name": "Acme Corporation"}'
```

### Duplicate a Company

```bash
curl -X POST http://localhost:3000/companies/YOUR_COMPANY_ID/duplicate
```

## WorkTypes

### List WorkTypes

```bash
curl http://localhost:3000/worktypes
```

### Create a WorkType

```bash
curl -X POST http://localhost:3000/worktypes \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Bug",
    "description": "A problem that impairs product functionality or user experience",
    "attributes": [
      {
        "name": "Severity",
        "data_type": "Numeric",
        "is_required": true,
        "is_hidden": false
      },
      {
        "name": "Steps to Reproduce",
        "data_type": "String",
        "is_required": true,
        "is_hidden": false
      },
      {
        "name": "Reported By",
        "data_type": "String",
        "is_required": false,
        "is_hidden": true
      }
    ]
  }'
```
