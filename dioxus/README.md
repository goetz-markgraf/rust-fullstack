# Dioxus Example

This app uses Dioxus in the full stack mode to create a simple todo app.

## Running the example

### Prerequisites

Dioxus comes with a cli tool that you can use to create new projects and
to run them. This example also uses Tailwind.

1. Install dioxus CLI: `cargo install dioxus-cli` (https://dioxuslabs.com/learn/0.6/getting_started)
2. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
3. Install the tailwind css cli: https://tailwindcss.com/docs/installation

## Run tailwind in watch mode

This example uses tailwind css. To compile the tailwind css, run:

```sh
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

## Run the app

To run the app, run:

```sh
dx serve
```

Then open your browser and go to `http://localhost:8080`.

# Explanations

The app is build in 'full stack' mode, that means, that frontend and backend
are in one project. There is no need to explicitly define an API or use
an HTTP client; this is all handled by Dioxus.

# Main Source Files

- `src/main.rs`: The main file of the app. Contains the router
- `src/not_found.rs`: The 404 page
- `src/todo_app`: The module that contains the real appl
  - `src/todo_app/types.rs`: The `Todo` struct
  - `src/todo_app/server_functions.rs`: all functions that are only run on the server and called from the frontend
  - `src/todo_app/app.rs`: The todo app's frontend. This file contains multiple components is one file
