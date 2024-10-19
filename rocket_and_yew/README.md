# Rocket and Yew Example

This is a simple example of a web application using Rocket and Yew.

It is layed out as a workspace with three projects:
- backend: The Rocket server
- frontend: The Yew frontend
- shared: Shared code between the frontend and backend

## Running the example

### Prerequisites

To run the frontend, you need to install Trunk first. Trunk is a
WebAssembly build tool that is used to build the frontend. It also
provides a development server with hot reloading.

```sh
cargo install trunk
```

### Run the backend

In the `backend` directory, run:

```sh
cargo run
```

### Run the frontend

In the `frontend` directory, run:

```sh
trunk serve
```

Then open your browser and go to `http://127.0.0.1:8080`.

*Attention!* Using `http://localhost:8080` will not work because the frontend
tries to access `127.0.0.1` which will result in a CORS error.
