mod models;
mod runner;

use std::net::SocketAddr;

use env_logger::Env;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use models::Scenario;
use tokio::sync::mpsc::{self, UnboundedSender};
use warp::{filters::ws::{Message, WebSocket}, reply::Reply, Filter};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct WebSocketParams {
    scenario_id: String,
}

pub(crate) async fn scenario_simulator(scenario: &Scenario, ws_sender: UnboundedSender<Message>) {

}

pub(crate) async fn handle_connection(ws: WebSocket, params: WebSocketParams) {
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    let (websocket_writer, mut websocket_outbound_stream) = mpsc::unbounded_channel();

    // Every time we get a message from the outbound stream, send it to the user.
    tokio::spawn(async move {
        while let Some(msg) = websocket_outbound_stream.recv().await {
            match user_ws_tx.send(msg).await {
                Ok(_) => (),
                Err(e) => {
                    error!("Error sending message to WebSocket: {}", e);
                    break;
                }
            }
        }
    });

    // Initial message writing
    let ws_writer_clone = websocket_writer.clone();

    tokio::spawn(async move {
        // TODO: some initial messages for setup
        // let _ = ws_writer_clone.send(message);

        // TODO: Start the scenario simulation
        // scenario_simulator(ws_writer_clone).await;
    });

    log::info!(
        "Connected WebSocket connection for scenario id {}",
        params.scenario_id,
    );

    // Every time we get a message from the user, handle it with the handler.
    while let Some(result) = user_ws_rx.next().await {
        match result {
            Ok(message) => {
                // TODO: Something
            }
            Err(e) => {
                error!("Error receiving message from WebSocket: {}", e);
                break;
            }
        }
    }

    info!(
        "WebSocket connection closed for scenario id {}",
        params.scenario_id,
    );
}

pub(crate) fn handle_ws_route(device_info: WebSocketParams, ws: warp::ws::Ws) -> impl Reply {
    // TODO: Set up / calculate the given scenario

    ws.on_upgrade(move |socket| handle_connection(socket, device_info))
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let web_server_port: u16 = std::env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse()
        .expect("PORT env variable must be a number");

    let ws_route = warp::path("ws")
        .and(warp::query::<WebSocketParams>())
        .and(warp::ws().map(|ws: warp::ws::Ws| ws.max_frame_size(64 << 20)))
        .map(handle_ws_route);

    let routes = ws_route;

    let addr: SocketAddr = ("[::]:".to_owned() + web_server_port.to_string().as_str())
        .parse()
        .unwrap();

    info!("Starting web server on port {}", web_server_port);
    warp::serve(routes).run(addr).await;
}
