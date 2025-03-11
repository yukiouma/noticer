use crate::executor::ExecutorManager;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

pub async fn serve(port: u16, manger: Arc<ExecutorManager>) -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let router = Router::new()
        .route("/api/execute", get(execute))
        .with_state(Arc::clone(&manger))
        .layer(TraceLayer::new_for_http());
    info!("Executor Server is running on port {}", port);
    axum::serve(listener, router).await?;
    Ok(())
}

async fn execute(
    manager: State<Arc<ExecutorManager>>,
    Query(request): Query<ExecuteRequest>,
) -> Result<Json<ExecuteReply>, AppError> {
    let id = request.id;
    let (code, message) = match manager.execute(id).await {
        Ok(_) => (0, "".to_string()),
        Err(e) => (1, e.to_string()),
    };
    Ok(Json(ExecuteReply { code, message }))
}

#[derive(Debug, Deserialize)]
struct ExecuteRequest {
    id: usize,
}

#[derive(Debug, Serialize)]
struct ExecuteReply {
    // 0 stands for success, else failure
    code: usize,
    message: String,
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}
