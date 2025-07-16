use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use dstack_sdk::dstack_client::DstackClient;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;

// A shared state for our application that holds the D-Stack client.
#[derive(Clone)]
struct AppState {
    client: Arc<DstackClient>,
}

// The response structure for the /quote endpoint.
#[derive(Serialize)]
struct QuoteResponse {
    quote: String,
    event_log: Vec<u8>,
}

#[tokio::main]
async fn main() {
    // Initialize the D-Stack client. It will use the default socket path
    // or the DSTACK_SIMULATOR_ENDPOINT environment variable if set.
    let client = DstackClient::new(None);

    // Create the application state.
    let app_state = AppState {
        client: Arc::new(client),
    };

    // Define the application routes.
    let app = Router::new()
        .route("/quote", get(get_quote))
        .route("/info", get(get_info))
        .with_state(app_state);

    // Start the server.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Handler for the `/quote` endpoint.
/// Returns a TEE quote and the corresponding event log.
async fn get_quote(State(state): State<AppState>) -> impl IntoResponse {
    match state.client.get_quote(vec![]).await {
        Ok(quote_data) => {
            let response = QuoteResponse {
                quote: quote_data.quote,
                event_log: quote_data.event_log,
            };
            (StatusCode::OK, Json(response).into_response())
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })).into_response(),
        ),
    }
}

/// Handler for the `/info` endpoint.
/// Returns the `app_compose` information from the TCB info.
async fn get_info(State(state): State<AppState>) -> impl IntoResponse {
    match state.client.info().await {
        Ok(info) => {
            // Serialize the info response to a generic JSON value to extract the desired field.
            match serde_json::to_value(info) {
                Ok(info_json) => {
                    if let Some(app_compose) = info_json
                        .get("tcb_info")
                        .and_then(|ti| ti.get("app_compose"))
                    {
                        (StatusCode::OK, Json(app_compose.clone()).into_response())
                    } else {
                        (
                            StatusCode::NOT_FOUND,
                            Json(serde_json::json!({
                                "error": "app_compose not found in tcb_info"
                            }))
                            .into_response(),
                        )
                    }
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": e.to_string() })).into_response(),
                ),
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })).into_response(),
        ),
    }
}

