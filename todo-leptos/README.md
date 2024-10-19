# Leptos SSR Example

This is a simple example of a web application using Leptos for server-side rendering.

## Running the example

### Prerequisites
To run the frontend, you need to install Trunk first. Trunk is a
WebAssembly build tool that is used to build the frontend. It also
provides a development server with hot reloading.

```sh
cargo install trunk
```

You also need to install the leptos CLI-tool that simplifies the
setup of a new SSR project as well as running the application.

```sh
cargo install cargo-leptos
```

### Run the application

In the root directory, run:

```sh
cargo leptos watch
```

Then open your browser and go to `http://localhost:3000`.
```

The project is run in a watch mode that means that every change (server
or client) will trigger a rebuild and the browser will automatically
refresh.

This also includes the SCSS style file `style/main.scss`. Adding
new styles and using them in the code will also automatically trigger a
rebuild and refresh.
