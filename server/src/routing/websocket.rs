/// Register the ws handler
pub fn register_ws(
    router: axum::Router<std::sync::Arc<crate::state::State>>,
) -> axum::Router<std::sync::Arc<crate::state::State>> {
    let router = router.route("/ws", axum::routing::get(ws_handler));
    router
}

async fn ws_handler(
    ws: axum::extract::WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::state::State>>,
) -> axum::response::Response {
    let receiver = state.subscribe();
    ws.on_upgrade(move |socket| handle_socket(socket, receiver))
}

async fn handle_socket(
    mut socket: axum::extract::ws::WebSocket,
    mut receiver: tokio::sync::broadcast::Receiver<api::server_ws::ServerEvent>,
) {
    loop {
        match receiver.recv().await {
            Ok(msg) => {
                let message = match serde_json::to_string(&msg) {
                    Ok(message) => message,
                    Err(_) => break,
                };
                let message = axum::extract::ws::Message::Text(message.into());
                match socket.send(message).await {
                    Ok(_) => (),
                    Err(_) => break,
                }
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                tracing::warn!("Server missed {n} message")
            }
        }
    }
}
