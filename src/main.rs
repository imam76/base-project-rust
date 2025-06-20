use anyhow::{Context, Result};
use axum::{Router, response::Redirect, routing::get};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().context("Failed to load .env file")?;

    let port: u16 = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .context("Failed to parse PORT environment variable")?;

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener: TcpListener = TcpListener::bind(addr).await?;

    println!("Listening on port {}", addr);

    axum::serve(listener, app().into_make_service()).await?;
    Ok(())
}

fn app() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/hi", get(hello_world))
        .route("/json", get(handle_get_json))
        .fallback(anything_else)
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn handle_get_json() {}

async fn anything_else() -> Redirect {
    Redirect::to("/hi")
}
