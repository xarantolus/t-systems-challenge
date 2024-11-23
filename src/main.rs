mod matching;
mod models;
mod runner;

use std::{convert::Infallible, net::SocketAddr};

use env_logger::Env;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use models::Scenario;
use runner::RunnerClient;
use serde::Serialize;
use tokio::sync::mpsc::{self, UnboundedSender};
use warp::{
    filters::ws::{Message, WebSocket},
    reject::Rejection,
    reply::Reply,
    Filter,
};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct WebSocketParams {
    scenario_id: String,
}

pub(crate) async fn scenario_simulator(scenario: &Scenario, ws_sender: UnboundedSender<Message>) {
    // First of all
}

pub(crate) async fn handle_connection(
    ws: WebSocket,
    initial_scenario: Scenario,
    runner_client: RunnerClient,
    params: WebSocketParams,
) {
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
        let _ = ws_writer_clone.send(
            initial_scenario
                .try_into()
                .expect("json serialization failed in initial write"),
        );

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

#[derive(Debug, Serialize, Clone)]
struct ErrorMsg {
    message: String,
}
impl warp::reject::Reject for ErrorMsg {}

pub(crate) async fn handle_ws_route(
    params: WebSocketParams,
    runner_client: RunnerClient,
    ws: warp::ws::Ws,
) -> Result<impl Reply, Rejection> {
    // Import the scenario from the database into the scenario runner simulation
    let initial_scenario = match runner_client.initialize_scenario(&params.scenario_id).await {
        Ok(s) => s,
        Err(e) => {
            let custom_error = ErrorMsg {
                message: format!("Failed to initialize scenario: {}", e),
            };
            return Err(warp::reject::custom(custom_error));
        }
    };

    let response = ws.on_upgrade(move |socket| {
        handle_connection(socket, initial_scenario, runner_client, params)
    });

    Ok(response)
}
fn with_client(
    client: RunnerClient,
) -> impl Filter<Extract = (RunnerClient,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let web_server_port: u16 = std::env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse()
        .expect("PORT env variable must be a number");

    let runner_base_url =
        std::env::var("RUNNER_BASE_URL").unwrap_or("http://localhost:8090/".to_string());
    let runner_client = RunnerClient::new(&runner_base_url);

    let ws_route = warp::path("ws")
        .and(warp::query::<WebSocketParams>())
        .and(with_client(runner_client))
        .and(warp::ws().map(|ws: warp::ws::Ws| ws.max_frame_size(64 << 20)))
        .and_then(handle_ws_route);

    let routes = ws_route;

    let addr: SocketAddr = ("[::]:".to_owned() + web_server_port.to_string().as_str())
        .parse()
        .unwrap();

    info!("Starting web server on port {}", web_server_port);
    warp::serve(routes).run(addr).await;
}
