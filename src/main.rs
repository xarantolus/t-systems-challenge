use core::panic;
use std::{cmp::min, convert::Infallible, error::Error, net::SocketAddr};
use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};

use env_logger::Env;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use serde::Serialize;
use tokio::{
    sync::mpsc::{self, Sender},
    time::sleep,
};
use warp::{
    Filter,
    filters::ws::{Message, WebSocket},
    reject::Rejection,
    reply::Reply,
};

use backend::BackendClient;
use models::{Scenario, UpdateScenario, UpdateVehicle};
use runner::RunnerClient;

mod backend;
pub mod matching;
mod models;
mod runner;

#[derive(Debug, serde::Deserialize)]
enum Algorithm {
    Nearest,
    ALSN,
}

impl PartialEq for Algorithm {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Algorithm::Nearest, Algorithm::Nearest) => true,
            (Algorithm::ALSN, Algorithm::ALSN) => true,
            _ => false,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct WebSocketParams {
    scenario_id: String,
    speed: Option<f64>,
    algorithm: Option<Algorithm>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScenarioCreationParams {
    number_of_vehicles: u64,
    number_of_customers: u64,
}

pub fn update_scenario_first(scenario: &Scenario) -> UpdateScenario {
    // For each non-assigned vehicle, we assign the next available customer
    let mut vehicle_assignments: Vec<UpdateVehicle> = Vec::new();

    let (mut available_vehicles, unavailable_vehicles): (Vec<_>, Vec<_>) = scenario
        .vehicles
        .iter()
        .partition(|vehicle| vehicle.customer_id.is_none());

    let riding_customer_id_set = unavailable_vehicles
        .iter()
        .filter_map(|v| v.customer_id.clone())
        .collect::<std::collections::HashSet<_>>();

    for customer in scenario.customers.iter() {
        if riding_customer_id_set.contains(&customer.id) || !customer.awaiting_service {
            continue;
        }

        let vehicle = available_vehicles
            .iter()
            .min_by_key(|v| {
                let dx = v.coord_x - customer.coord_x;
                let dy = v.coord_y - customer.coord_y;
                (dx * dx + dy * dy).abs() as u64
            });
        let Some(vehicle) = vehicle else { break; };

        vehicle_assignments.push(UpdateVehicle {
            id: vehicle.id.clone(),
            customer_id: vehicle.customer_id.clone().unwrap_or(customer.id.clone()),
        });
        available_vehicles.swap_remove(
            available_vehicles
                .iter()
                .position(|v| v.id == vehicle.id)
                .unwrap(),
        );
    }

    UpdateScenario {
        vehicles: vehicle_assignments,
    }
}

pub(crate) async fn scenario_simulator(
    runner_client: RunnerClient,
    mut scenario: Scenario,
    ws_sender: Sender<Message>,
    speed: f64,
    algorithm: Algorithm,
) -> Result<(), Box<dyn Error>> {
    let scenario_id = scenario.id.clone();

    let mut precomputed_assignments: HashMap<String, VecDeque<String>> = HashMap::new();
    if algorithm == Algorithm::ALSN {
        precomputed_assignments = todo!(); //matching::compute_assignment(&scenario);
    }
    let scenario_launch = match runner_client.launch_scenario(&scenario_id, speed).await {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("Failed to launch scenario: {}", e).into());
        }
    };

    info!("Scenario launched: {:?}", scenario_launch);

    while scenario.end_time.is_none() {
        let assignments = match algorithm {
            Algorithm::Nearest => update_scenario_first(&scenario),
            Algorithm::ALSN => UpdateScenario {
                vehicles: precomputed_assignments
                    .iter()
                    .filter_map(|(v, customer)| {
                        customer.front().map(|c| UpdateVehicle {
                            id: v.clone(),
                            customer_id: c.clone(),
                        })
                    })
                    .collect(),
            },
        };
        debug_assert!(
            assignments.vehicles.len() <= min(scenario.vehicles.len(), scenario.customers.len())
        );
        let update = runner_client
            .update_scenario(&scenario_id, &assignments)
            .await?;

        if algorithm == Algorithm::ALSN {
            for x in update.updated_vehicles {
                precomputed_assignments.get_mut(&x.id).unwrap().pop_front();
            }
        }

        #[cfg(debug_assertions)]
        if !update.failed_to_update.is_empty() {
            // Dump all info to stdout before panicking
            println!(
                "Scenario: {}",
                serde_json::to_string_pretty(&scenario).unwrap()
            );
            println!(
                "Assignments: {}",
                serde_json::to_string_pretty(&assignments).unwrap()
            );
            println!(
                "Failed to Update (Vehicle IDs): {}",
                serde_json::to_string_pretty(&update.failed_to_update).unwrap()
            );

            panic!("Wrong update sent for some vehicles!");
        }

        scenario = runner_client.get_scenario(&scenario_id).await?;

        let Ok(_) = ws_sender.send((&scenario).try_into()?).await else {
            return Err("WebSocket disconnected".into());
        };

        // TODO: Maybe not?
        // sleep(std::time::Duration::from_millis(100)).await;
    }

    Ok(())
}

pub(crate) async fn handle_connection(
    ws: WebSocket,
    initial_scenario: Scenario,
    runner_client: RunnerClient,
    params: WebSocketParams,
) {
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    let (websocket_writer, mut websocket_outbound_stream) = mpsc::channel(1);

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
    let initial_scenario_clone = initial_scenario.clone();
    tokio::spawn(async move {
        // TODO: some initial messages for setup
        let _ = ws_writer_clone.send(
            (&initial_scenario)
                .try_into()
                .expect("json serialization failed in initial write"),
        );

        scenario_simulator(
            runner_client,
            initial_scenario_clone,
            ws_writer_clone,
            params.speed.unwrap_or(0.033f64),
            params.algorithm.unwrap_or(Algorithm::Nearest),
        )
        .await
        .expect("Failed to run scenario simulation")
    });

    log::info!(
        "Connected WebSocket connection for scenario id {}",
        params.scenario_id,
    );

    // Every time we get a message from the user, handle it with the handler.
    while let Some(result) = user_ws_rx.next().await {
        match result {
            Ok(_message) => {
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

pub(crate) async fn create_scenario(
    params: ScenarioCreationParams,
    backend_client: BackendClient,
) -> Result<impl Reply, Rejection> {
    let response = backend_client
        .create_scenario(params.number_of_vehicles, params.number_of_customers)
        .await;

    match response {
        Ok(scenario) => Ok(warp::reply::json(&scenario)),
        Err(e) => {
            let custom_error = ErrorMsg {
                message: format!("Failed to create scenario: {}", e),
            };
            Err(warp::reject::custom(custom_error))
        }
    }
}

fn with_runner_client(
    client: RunnerClient,
) -> impl Filter<Extract = (RunnerClient,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

fn with_backend_client(
    client: BackendClient,
) -> impl Filter<Extract = (BackendClient,), Error = Infallible> + Clone {
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
        std::env::var("RUNNER_BASE_URL").unwrap_or("http://localhost:8090".to_string());
    let runner_client = RunnerClient::new(&runner_base_url);

    let backend_base_url =
        std::env::var("BACKEND_BASE_URL").unwrap_or("http://localhost:8080".to_string());
    let backend_client = BackendClient::new(&backend_base_url);

    let create_scenario_route = warp::path!("scenario" / "create")
        .and(warp::post())
        .and(warp::query::<ScenarioCreationParams>())
        .and(with_backend_client(backend_client))
        .and_then(create_scenario);

    let ws_route = warp::path("ws")
        .and(warp::query::<WebSocketParams>())
        .and(with_runner_client(runner_client))
        .and(warp::ws().map(|ws: warp::ws::Ws| ws.max_frame_size(64 << 20)))
        .and_then(handle_ws_route);

    let routes = ws_route.or(create_scenario_route);

    let addr: SocketAddr = ("[::]:".to_owned() + web_server_port.to_string().as_str())
        .parse()
        .unwrap();

    info!("Starting web server on port {}", web_server_port);
    warp::serve(routes).run(addr).await;
}
