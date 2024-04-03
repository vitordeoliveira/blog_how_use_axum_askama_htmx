use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {
    title: &'static str,
    myownvalue: &'static str,
}

async fn root() -> impl IntoResponse {
    let root = RootTemplate {
        title: "title",
        myownvalue: "Hello from AXUM+ASKAMA+HTMX",
    };
    (StatusCode::OK, Html(root.render().unwrap()))
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    title: &'static str,
    stringvalue: &'static str,
    vec_strings: Vec<&'static str>,
}

async fn home() -> impl IntoResponse {
    let home = HomeTemplate {
        title: "title",
        stringvalue: "Hello from myownvalue",
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
    };
    (StatusCode::OK, Html(home.render().unwrap()))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt().init();
    let port = "8080";

    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/home", get(home));

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
