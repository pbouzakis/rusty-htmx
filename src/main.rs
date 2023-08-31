use axum::{
    middleware,
    response::{Html, Response},
    routing::{get, post},
    Router,
    http::{Method, Uri},
};
use minijinja::{path_loader, context, Environment};
use once_cell::sync::Lazy;
use tower_http::services::ServeDir;
use tower_livereload::{LiveReloadLayer, predicate::Predicate};
use uuid::Uuid;
use shop::fetch_catalog;
use log::log_request;

mod shop;
mod log;

static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.add_filter("price", display_price);
    env.set_loader(path_loader("templates"));
    env
});

// Do not support livereload on htmx requests
// This prevents browser from crashing due to too many livereload event listeners on the page.
#[derive(Copy, Clone, Debug)]
pub struct DoNotReloadOnPartialHtmls;

impl<T> Predicate<http::Request<T>> for DoNotReloadOnPartialHtmls {
    fn check(&mut self, request: &http::Request<T>) -> bool {
        !request.headers().contains_key("Hx-Request")
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("templates/_CLIENT_/assets"))
        .nest_service("/media", ServeDir::new("client/media"))
        .route(
            "/",
            get(home),
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
        .layer(
            middleware::map_response(mw_response)
        );

    #[cfg(debug_assertions)]
    let app = app.layer(
        LiveReloadLayer::new()
            .request_predicate::<axum::body::Body, DoNotReloadOnPartialHtmls>(DoNotReloadOnPartialHtmls)
    );        

    // run it with hyper
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn mw_response(
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
	println!("->> MIDDLEWARE:: mw_response");
    
    log_request(
        Uuid::new_v4(),
        req_method,
        uri,
        res.status(),
    );

    res
}

async fn home() -> Html<String> {
    let tmpl = ENV.get_template("home.html").unwrap();
    let ctx = context!();    
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
    let ctx = context!(
        catalog => catalog,
        cart_count => 0,
    );
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
    let response = format!(
        "<div>Added!</div><span id=\"cart-count\" hx-swap-oob=\"true\">{}</span>", 
        count
    );
    Html(response)
}

fn display_price(price: f32) -> String {
    format!("${:.2}", price)
}
