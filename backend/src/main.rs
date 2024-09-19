#[allow(unused_imports)]
use anyhow::{Context, Result};
use axum::{response::Redirect, routing::get, Router};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing (Logger)
    tracing_subscriber::fmt::init();

    // Socket IO Setup
    let (io_layer, io) = SocketIo::new_layer();
    io.ns("/", socket_connection_handler);

    // Cors Stuff
    let cors_layer = CorsLayer::permissive();

    // Axum Setup
    let app = Router::new()
        .route(
            "/",
            get(Redirect::temporary(
                "https://github.com/WieSeTechnologies/alert-system",
            )),
        )
        .layer(ServiceBuilder::new().layer(cors_layer).layer(io_layer));

    // Server
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[allow(unused)]
// Data Argument must be of type Value from Serde Json
fn socket_connection_handler(socket: SocketRef) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    socket.on("send-trigger", trigger_alert);
}

#[allow(unused)]
fn trigger_alert(socket: SocketRef, Data(data): Data<String>) {
    info!("Alert triggered!");
    socket.broadcast().emit("trigger-alert", true).unwrap();
    socket.emit("success", "Altert triggered!").unwrap();
}
