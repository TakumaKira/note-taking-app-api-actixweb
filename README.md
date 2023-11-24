# note-taking-app-api-actixweb

- [note-taking-app-api-actixweb](#note-taking-app-api-actixweb)
  - [The task from ChatGPT](#the-task-from-chatgpt)
  - [Getting Started](#getting-started)

## The task from ChatGPT

This API will allow users to create, read, update, and delete notes. Here's a basic specification:

API Specification: Note-Taking Application
API Base URL: https://yourapi.com/api/v1

Endpoints:

a. Create a Note

b. Get All Notes

- Method: GET
- Endpoint: /notes
- Response:
json [ { "id": "unique-note-id", "title": "string", "content": "string", "createdAt": "timestamp" } ]

c. Get a Single Note

- Method: GET
- Endpoint: /notes/{noteId}
- Response:
json { "id": "unique-note-id", "title": "string", "content": "string", "createdAt": "timestamp" }

d. Update a Note

- Method: PUT
- Endpoint: /notes/{noteId}
- Body:
json { "title": "string", "content": "string" } - Response:
json { "id": "unique-note-id", "title": "string", "content": "string", "updatedAt": "timestamp" }

e. Delete a Note

- Method: DELETE
- Endpoint: /notes/{noteId}
- Response:
json { "message": "Note deleted successfully." }

Authentication:

For simplicity, we can omit authentication. However, in a real-world scenario, you would likely add an authentication layer to protect the data.
Error Handling:

Make sure to handle common HTTP errors (e.g., 404 Not Found, 500 Internal Server Error) with appropriate error messages.
This API is quite basic and meant for practice. In a real-world scenario, you would need to consider more complex elements like authentication, more robust error handling, data validation, and possibly rate limiting.

## Getting Started

Before running any command remember to export the DATABASE_URL environment variable.

```bash
export DATABASE_URL=sqlite://$PWD/notes.db
```

Run the application.

```bash
make run
```

Run the test suite:

```bash
make test
```

Visit the swagger ui:

```bash
http://localhost:8081/swagger-ui/
```
