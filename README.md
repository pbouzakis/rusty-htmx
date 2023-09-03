# rusty-htmx

A proof of concept web server to explore the following
* Learn rust
* Learn [htmx](https://htmx.org/)
* Embracing RESTful archiceture, Hypermedia, and HATEOAS 
* Find a suitable web framework (Default is axum)
* Find a suitable html template engine (type-safe)
* Determine how to serve css (assummning tailwind css)
* UI component library / design system to build upon
* Minimize client side js but have a defined path when needed
* A enjoyable developer experience building an app that spans client and server.

## Template Engine

### minijinja
Port of Python's jinja.

#### Future spikes
* Aksama
* There is a jsx template engine?
* Maud
* Tera (also jinja inspired)

## Web Framework
Defaulting to axum as it is the current library being used in the production app that spear-headed this POC.

### Axum
axum is designed to work with tokio and hyper. Runtime and transport layer independence is not a goal, at least for the time being.

### Frontend Frameworks

#### CSS libraries
- TailwindCSS
- XtendUI
- Flowbite

#### Future spikes
* Leptos (specifically Leptos SSR)
* Yew (is there SSR support?)


#### Links
* https://docs.rs/axum/latest/axum/
* https://tokio.rs/blog/2021-07-announcing-axum
* [Tokio: asynchronous runtime](https://tokio.rs/tokio/tutorial)


### Reloading
A baseline for modern web development is the ability to automatically reload the server and client on changes of the source code.
On the client, this should also include live reloading where applicable.

#### cargo-watch
This should allow us to restart our axum web server:

`cargo watch -x run`


#### tower-livereload
This crate should allow us to live reload the browser so we don't need to manually refresh after the server reloads.
  