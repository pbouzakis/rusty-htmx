use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
    http::HeaderMap, 
    extract::{State, Form},
};
use http::HeaderValue;
use minijinja::context;
use serde::Deserialize;

use crate::session::SessionController;
use crate::ENV;
use crate::shop::gateway;

pub fn routes() -> Router<SessionController> {
    Router::new()
        .route("/cart", get(view_cart))
        .route("/shop", get(view_store))
        .route("/shop/cart", post(add_to_cart))           
}

async fn view_cart(
    State(session): State<SessionController>, 
    headers: HeaderMap
) -> Html<String> {
    let tmpl = ENV.get_template("cart.html").unwrap();
    
    // Temp to show loading
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let cart = session.cart();

    let ctx = context!(
        cart_items => cart.items(),
        cart_count => cart.count(),
    );

    if headers.contains_key("Hx-Request") {
        let r = tmpl
            .eval_to_state(ctx).unwrap()
            .render_block("body").unwrap(); 

        Html(r)          

    } else {
        let r = tmpl.render(ctx).unwrap(); 
        Html(r)  
    }
}

async fn view_store(State(session): State<SessionController>) -> Html<String> {
    let tmpl = ENV.get_template("shop.html").unwrap();
    let cart = session.cart();
    let catalog = gateway::fetch_catalog();

    let ctx = context!(
        catalog => catalog,
        cart_count => cart.count(),
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
    
    let mut cart = session.cart();
    let product = gateway::fetch_product_by_slug(&params.sku);
    let count = cart.add(&product);

    //let updated_cart_count = session.update_cart(params.sku);

    let mut headers = HeaderMap::new();
    headers.insert("HX-Trigger", HeaderValue::from_str("cart-updated").unwrap());

    let ctx = context!(
        updated_cart_count => count,
    );

    let r: String = tmpl.render(ctx).unwrap(); 
    (headers, Html(r))
}
