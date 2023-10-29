#![allow(unused)] // for beginning only

use axum::extract::{Path, Query};
// cargo watch -q -c -w src/ -x run
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use rust_axum_intro::error::{Error, Result};
use rust_axum_intro::web::routes_login;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes_fib())
        .merge(routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region: --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("->> Listening on {}\n", addr);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
        

    // endregion: --- Start Server
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper\n\n", "RES_MAPPER");

    res
}

// region: --- RoutesStatic

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// endregion: --- RoutesStatic

// region: --- RoutesHello

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_haello2))
}

// region: --- Handler Hello

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., /hello?name=Alito
async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    println!("-->>{:<5} - handler_hello - {:?}", "HANDLER", params);

    let name = params.0.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{}!!!!</strong>", name))
}

// e.g, /hello2/Polito

async fn handler_haello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-->>{:<5} - handler_hello2 - {:?}", "HANDLER", name);

    Html(format!("Hello <strong>{}!!!!</strong>", name))
}

// endregion: --- Handler Hello

// endregion: --- RoutesHello

// region: -- Fib

fn fib(f: u32) -> u128 {
    let mut n1: u128 = 0;
    let mut n2: u128 = 1;
    let mut bandera: i8 = 1;

    for _ in 0..f - 2 {
        if bandera == 1 {
            n1 += n2;
        } else {
            n2 += n1;
        }
        bandera *= -1;
    }

    if bandera == 1 {
        n2
    } else {
        n1
    }
}

#[derive(Debug, Deserialize)]
struct FibParams {
    number: Option<u32>,
}

fn routes_fib() -> Router {
    Router::new().route("/fib", get(handler_fib))
    //.route("/hello2/:name", get(handler_haello2))
}

async fn handler_fib(Query(params): Query<FibParams>) -> impl IntoResponse {
    println!("-->>{:<5} - handler_fib - {:?}", "HANDLER", params);

    let n = params.number.unwrap();

    let chars: Vec<char> = fib(n).to_string().chars().collect();
    let mut result = String::new();

    let len = chars.len();
    for (i, ch) in chars.into_iter().enumerate() {
        if (len - i) % 3 == 0 && i != 0 {
            result.push(',');
        }
        result.push(ch);
    }

    Html(format!("Fib of {}: {}", n, result))
}

// endregion: -- Fib
