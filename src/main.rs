use qnabot::{agent, routes};

use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let agent = agent::build_agent("dataset/qna.md").await?;

    // Start web server
    let app = routes::create_app(agent);
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("server running on http://localhost:{}", port);
    axum::serve(listener, app).await?;

    Ok(())
}
