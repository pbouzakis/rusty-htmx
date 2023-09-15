use axum::{
    middleware,
    response::Response,
    Router,
    http::{Method, Uri},
};
use minijinja::{path_loader, Environment};
use once_cell::sync::Lazy;
use tower_http::services::ServeDir;
use tower_livereload::{LiveReloadLayer, predicate::Predicate};
use uuid::Uuid;
use log::log_request;

mod session;
mod marketing;
mod shop;
mod log;

use session::SessionController;

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
        .merge(marketing::routes())
        .merge(shop::web::routes())
        .with_state(SessionController::new())
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

fn display_price(price: f32) -> String {
    format!("${:.2}", price)
}
