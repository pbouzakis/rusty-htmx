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
* There is a jsx template engine?
* Tera (also jinja inspired)

## Web Framework
Defaulting to axum as it is the current library being used in the production app that spear-headed this POC.

### Axum
axum is designed to work with tokio and hyper. Runtime and transport layer independence is not a goal, at least for the time being.

### Frontend Framworks
Originially designed for the frontend, however, with the foundation of this POC based on htmx, do we need this? (If this was a JS app, would we still need react?)

#### Future spikes
* Leptos (specifically Leptos SSR)
* Yew (is there SSR support?)


#### Links
* https://docs.rs/axum/latest/axum/
* https://tokio.rs/blog/2021-07-announcing-axum
* [Tokio: asynchronous runtime](https://tokio.rs/tokio/tutorial)


### Reloading
A baseline for modern web development is the ability to automatically reload the server and client on changes of the source code.
On the client, this should also include hot reloading where applicable.

#### cargo-watch
This should allow us to restart our axum web server:

`cargo watch -x run`



  