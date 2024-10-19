# Beyond Javascript -- Writing a Web Application Full Stack with Rust

Repository for the talk at c't webdev 2024

This repository contains code that demonstrates the usage of
Rust to develop a full stack web application.

The application is a simple Todo-List that stores its data
in a flat file in Json format.


# Prerequisites

In order to run the examples, you need to have Rust installed.

That can be done by running the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After that, you need to install the nightly toolchain, as the
examples use some features that are not yet stable.

```bash
rustup toolchain install nightly
```

You also need to install the webassembly target:

```bash
rustup target add wasm32-unknown-unknown
```

Learn more about the Rust Programming Language at [rust-lang.org](https://www.rust-lang.org/)

# The Examples

There are three different implementations using different tech stacks. In each
of the three folders, you'll find a `README.md` that explains the architecture and
how to run the example.

## `rocket_and_yew`

This directory contains a rather classical setup with different
projects for the backend and the frontend. Additionally, you'll find
a shared library that contains the definition of the data transfer objects,
that are also used as a primary data model in both the frontend and the backend.

Learn more about Rocket at [rocket.rs](https://rocket.rs/)

Learn more about Yew at [yew.rs](https://yew.rs/)

## `todo-leptos`

This directory contains a single project that uses the Leptos web framework for
both the backend and the frontend in a single code base. This is called
"Leptos SSR" (Server Side Rendering).

Leptos is inspired by solid-js rather than react.

Learn more about Leptos at [leptos.dev](https://leptos.dev/)

## `dioxus`

This directory has a similar setup as the `rocket_and_yew` example, but uses the
Dioxus web framework for the backend. Dioxus is a web framework that also can
be used in a list of different setups, that include:

- Single Page Applications (dioxus web)
- Server Side Rendering (dioxus fullstack)
- Native Desktop Applications (dioxus desktop)
- Native Mobile Applications (dioxus mobile)

Learn more about Dioxus at [dioxuslabs.org](https://dioxuslabs.com/)
