use axum::{extract::Path, response::Html, routing::get, Router};
use minijinja::{path_loader, context, render, Environment};
use once_cell::sync::Lazy;
use serde::Serialize;

static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
});

#[derive(Debug, Serialize)]
struct Item {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize)]
struct Topic {
    key: String,
    items: Vec<Item>,
}

const HOME: &'static str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>Rusty HTMX</title>
  <meta name="description" content="A lil rust, a lil htmx, and a lot of hope.">
  <meta name="author" content="Paul Bouzakis">
</head>

<body>
    <h1>Hello, World!</h1>
</body>
</html>
"#;

const TOPIC_TEMPLATE: &'static str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>Rusty HTMX</title>
  <meta name="description" content="A lil rust, a lil htmx, and a lot of hope.">
  <meta name="author" content="Paul Bouzakis">
</head>

<body>
    <h1>Topic: {{ topic.key|title }}</h1>
    <h2>Details</h2>
    <ul>
        {% for item in topic.items %}
        <li>{{ item.name }} ({{ item.id }})</li>
        {% endfor %}
    <ul>
</body>
</html>
"#;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(home),
        )
        .route(
            "/topic/:key",
            get(get_topic),
        )
        .route(
            "/about",
            get(about),
        )
        .route(
            "/info",
            get(get_info),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Html<String> {
    let r = render!(HOME);
    Html(r)
}

async fn get_topic(Path(topic): Path<String>) -> Html<String> {
    let item_example = vec![
        Item {
            id: 1,
            name: "Rust".into(),
        },
        Item {
            id: 2,
            name: "Htmx".into(),
        },
    ];
    let topic_example = Topic {
        key: topic,
        items: item_example,
    };
    let r = render!(TOPIC_TEMPLATE, topic => topic_example );
    Html(r)
}

async fn about () -> Html<String> {
    let tmpl = ENV.get_template("about.html").unwrap();
    let ctx = context!(name => "World");
    let r = tmpl.render(ctx).unwrap();
    Html(r)
}

async fn get_info() -> Html<&'static str> {
    Html("<h2>MORE INFO COMING SOON!")
}
