use axum::{
    middleware,
    response::{Html, Response, IntoResponse},
    routing::{get, post},
    Router,
    http::{Method, Uri, HeaderMap}, 
    extract::{State, Form},
};
use http::HeaderValue;
use minijinja::{path_loader, context, Environment};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_livereload::{LiveReloadLayer, predicate::Predicate};
use uuid::Uuid;
use log::log_request;

mod shop;
mod log;

use shop::gateway;
use shop::model::{Cart, CartItem};

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


#[derive(Clone)]
struct SessionController {
    cart: Arc<Mutex<Cart>>,
}

impl SessionController {
    fn new() -> Self {
        Self { 
            cart: Arc::new(
                Mutex::new(Cart {
                    items: vec![],
                })
            )
         }
    }
    fn cart_count(&self) -> usize {
        let cart = self.cart.lock().unwrap();
        cart.items.len()
    }
    fn update_cart(&self, sku: String) -> usize {
        let mut cart = self.cart.lock().unwrap();

        cart.items.push(sku);

        println!("{:#?}", cart.items);

        cart.items.len()
    }
    fn cart_items(&self) -> Vec<CartItem> {
        let catalog = gateway::fetch_catalog();
        let cart = self.cart.lock().unwrap();
        let mut items = vec![];

        for product in catalog {
            if cart.items.contains(&product.slug) {
                let quantity = cart.items.iter().filter(|&slug| *slug == product.slug).count();
                let total = product.price.clone() * quantity as f32;

                items.push(
                    CartItem {
                        product,
                        quantity,
                        total,
                    }
                )
            }
        }
        items
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
            "/cart", 
            get(view_cart)
        )
        .route(
            "/shop",
            get(view_store),
        )        
        .route(
            "/shop/cart",
            post(add_to_cart),
        )           
        .route(
            "/about",
            get(about),
        )
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

async fn home(State(session): State<SessionController>) -> Html<String> {
    let tmpl = ENV.get_template("home.html").unwrap();
    let ctx = context!(cart_count => session.cart_count());
    let r = tmpl.render(ctx).unwrap();
    Html(r)
}

async fn about(State(session): State<SessionController>) -> Html<String> {
    let tmpl = ENV.get_template("about.html").unwrap();
    let ctx = context!(cart_count => session.cart_count());

    let r = tmpl.render(ctx).unwrap();
    Html(r)
}

async fn view_cart(State(session): State<SessionController>, headers: HeaderMap) -> Html<String> {
    let tmpl = ENV.get_template("cart.html").unwrap();
    
    // Temp to show loading
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let ctx = context!(
        cart_items => session.cart_items(),
        cart_count => session.cart_count(),
        partial => headers.contains_key("Hx-Request"),
    );

    let r = tmpl.render(ctx).unwrap(); 
    Html(r)  
}

async fn view_store(State(session): State<SessionController>) -> Html<String> {
    let tmpl = ENV.get_template("shop.html").unwrap();
    let catalog = gateway::fetch_catalog();

    let ctx = context!(
        catalog => catalog,
        cart_count => session.cart_count(),
    );

    let r: String = tmpl.render(ctx).unwrap(); 
    Html(r)   
}

#[derive(Deserialize)]
struct AddToCartParams {
    sku: String,
}

async fn add_to_cart(
    State(session): State<SessionController>,
    Form(params): Form<AddToCartParams>, 
) -> impl IntoResponse {
    println!("Adding sku:{}", params.sku);

    let tmpl = ENV.get_template("cart-updated.html").unwrap();
    let updated_cart_count = session.update_cart(params.sku);

    let mut headers = HeaderMap::new();
    headers.insert("HX-Trigger", HeaderValue::from_str("cart-updated").unwrap());

    let ctx = context!(
        updated_cart_count => updated_cart_count,
    );

    let r: String = tmpl.render(ctx).unwrap(); 
    (headers, Html(r))
}

fn display_price(price: f32) -> String {
    format!("${:.2}", price)
}
