use crate::agent::Agent;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use rig::completion::Prompt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, services::ServeDir};

#[derive(Clone)]
pub struct AppState {
    pub agent: Arc<Agent>,
}

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    pub question: String,
}

#[derive(Debug, Serialize)]
pub struct AskResponse {
    pub answer: String,
}

pub fn create_app(agent: Agent) -> Router {
    let state = AppState {
        agent: Arc::new(agent),
    };
    let cors = CorsLayer::permissive();

    Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/health", get(health_handler))
        .route("/ask", post(ask_handler))
        .layer(cors)
        .with_state(state)
}

async fn health_handler() -> String {
    "OK".to_string()
}

async fn ask_handler(
    State(state): State<AppState>,
    Json(payload): Json<AskRequest>,
) -> Json<AskResponse> {
    let answer = state
        .agent
        .prompt(&payload.question)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Agent error: {}", e);
            "Sorry, I couldn't answer that.".to_string()
        });
    Json(AskResponse { answer })
}
