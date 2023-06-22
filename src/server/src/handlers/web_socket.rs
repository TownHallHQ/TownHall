use std::net::SocketAddr;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};

use tracing::info;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

async fn web_socket(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    info!("Received WebSocket Connection from {}", addr);
    todo!()
}
