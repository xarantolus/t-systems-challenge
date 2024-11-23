use serde::{Deserialize, Serialize};
use warp::filters::ws::Message;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    pub id: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub status: String,
    pub vehicles: Vec<Vehicle>,
    pub customers: Vec<Customer>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: String,
    pub coord_x: f64,
    pub coord_y: f64,
    pub destination_x: Option<f64>,
    pub destination_y: Option<f64>,
    pub awaiting_service: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub id: String,
    pub coord_x: f64,
    pub coord_y: f64,
    pub is_available: bool,
    pub vehicle_speed: Option<f64>,
    pub customer_id: Option<String>,
    pub remaining_travel_time: Option<f64>,
    pub distance_travelled: Option<f64>,
    pub active_time: Option<f64>,
    pub number_of_trips: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScenario {
    pub vehicles: Vec<UpdateVehicle>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVehicle {
    pub id: String,
    pub customer_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScenarioResponse {
    #[serde(default)]
    pub failed_to_update: Vec<String>,
    #[serde(default)]
    pub updated_vehicles: Vec<Vehicle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchScenarioResponse {
    pub message: String,
    pub scenario_id: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
}

// Allows sending a scenario as text via the WebSocket connection
impl TryFrom<&Scenario> for Message {
    type Error = anyhow::Error;

    fn try_from(scenario: &Scenario) -> Result<Self, Self::Error> {
        Ok(Message::text(serde_json::to_string(&scenario)?))
    }
}
