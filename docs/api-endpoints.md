# API Endpoints

This section provides a vrief description of the all endpoints. Check the [examples.md](examples.md) markdown file to see how to deal with them.

## Companies

| Method | Endpoint                  | Description                           |
|--------|---------------------------|---------------------------------------|
| GET    | /companies                | List all companies (with name filter) |
| POST   | /companies                | Create a new company                  |
| GET    | /companies/{id}           | Get a company by ID                   |
| PUT    | /companies/{id}           | Update a company                      |
| POST   | /companies/{id}/duplicate | Duplicate a company                   |

## WorkTypes

| Method | Endpoint                  | Description                           |
|--------|---------------------------|---------------------------------------|
| GET    | /worktypes                | List all worktypes (with name filter) |
| POST   | /worktypes                | Create a new worktype                 |
| GET    | /worktypes/{id}           | Get a worktype by ID                  |
| PUT    | /worktypes/{id}           | Update a worktype                     |
| POST   | /worktypes/{id}/duplicate | Duplicate a worktype                  |
