use axum::{
    response::Html,
    routing::get,
    Router,
    extract::State,
};
use minijinja::context;
use crate::ENV;
use crate::session::SessionController;

pub fn routes() -> Router<SessionController> {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
}

async fn home(State(session): State<SessionController>) -> Html<String> {
    let tmpl = ENV.get_template("home.html").unwrap();
    let cart = session.cart();
    let ctx = context!(cart_count => cart.count());
    
    let r = tmpl.render(ctx).unwrap();
    Html(r)
}

async fn about(State(session): State<SessionController>) -> Html<String> {
    let tmpl = ENV.get_template("about.html").unwrap();
    let cart = session.cart();
    let ctx = context!(cart_count => cart.count());

    let r = tmpl.render(ctx).unwrap();
    Html(r)
}
