use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub vehicles: Vec<Vehicle>,
    pub customers: Vec<Customer>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: String,
    pub coord_x: i64,
    pub coord_y: i64,
    pub destination_x: i64,
    pub destination_y: i64,
    pub awaiting_service: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub id: String,
    pub coord_x: i64,
    pub coord_y: i64,
    pub is_available: bool,
    pub vehicle_speed: i64,
    pub customer_id: String,
    pub remaining_travel_time: i64,
    pub distance_travelled: i64,
    pub active_time: i64,
    pub number_of_trips: i64,
}
