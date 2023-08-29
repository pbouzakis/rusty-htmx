use axum::{
  response::Html, 
  routing::{get, post},
  Router,
};
use minijinja::{path_loader, context, Environment};
use once_cell::sync::Lazy;
use serde::Serialize;
use tower_http::services::ServeDir;
use shop::fetch_catalog;

mod shop;

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
        .nest_service("/assets", ServeDir::new("templates/_CLIENT_/assets"))
        .nest_service("/images", ServeDir::new("client/images"))
        .route(
            "/",
            get(home),
        )
        .route(
            "/styled",
            get(styled),
        )
        .route(
            "/shop",
            get(shop),
        )        
        .route(
            "/shop/cart",
            post(add_to_cart),
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

async fn about() -> Html<String> {
    let tmpl = ENV.get_template("about.html").unwrap();
    let ctx = context!(name => "About");
    let r = tmpl.render(ctx).unwrap();
    Html(r)
}

async fn shop() -> Html<String> {
    let tmpl = ENV.get_template("shop.html").unwrap();
    let catalog = fetch_catalog();
    let ctx = context!(catalog => catalog);
    let r = tmpl.render(ctx).unwrap(); 
    Html(r)   
}

static mut COUNT: i32 = 0;
unsafe fn update_cart() -> &'static i32 {
    COUNT += 1;
    &COUNT
}

async fn add_to_cart() -> Html<String> {
    let count = unsafe { 
        update_cart()
    };
    let response = format!("<div>Added! Cart count: {}</div>", count);
    Html(response)
}

async fn styled() -> Html<String> {
    let tmpl = ENV.get_template("styled.html").unwrap();
    let ctx = context!();
    let r = tmpl.render(ctx).unwrap();
    Html(r)
}

async fn get_info() -> Html<&'static str> {
    Html("<div><img src=\"https://files.adrianistan.eu/htmx2.jpg\" /></div>")
}
