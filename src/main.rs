use axum::{extract::Path, response::Html, routing::get, Router};
use minijinja::{path_loader, context, Environment};
use once_cell::sync::Lazy;
use serde::Serialize;

static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
});

#[derive(Debug, Serialize)]
struct Link {
    display: String,
    href: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(home),
        )
        .route(
            "/about",
            get(about),
        )
        .route(
            "/info",
            get(get_info),
        );

    #[cfg(debug_assertions)]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());        

    // run it with hyper
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Html<String> {
    let hp_links = vec![
        Link {
            display: "Rust".into(),
            href: "https://doc.rust-lang.org/book/".into(),
        },
        Link {
            display: "Htmx".into(),
            href: "https://htmx.org/".into(),
        },
    ];

    let tmpl = ENV.get_template("home.html").unwrap();
    let ctx = context!(name => "Home", links => hp_links);    
    let r = tmpl.render(ctx).unwrap();
    Html(r)
}

async fn about () -> Html<String> {
    let tmpl = ENV.get_template("about.html").unwrap();
    let ctx = context!(name => "About");
    let r = tmpl.render(ctx).unwrap();
    Html(r)
}

async fn get_info() -> Html<&'static str> {
    Html("<h2>MORE INFO COMING SOON!")
}
