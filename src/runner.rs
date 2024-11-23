use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::models::{LaunchScenarioResponse, Scenario, UpdateScenario, UpdateScenarioResponse};

#[derive(Debug, Clone)]
pub struct RunnerClient {
    client: Client,
    base_url: String,
}

impl RunnerClient {
    pub fn new(runner_server_base_url: &str) -> Self {
        RunnerClient {
            client: Client::new(),
            base_url: runner_server_base_url.to_string(),
        }
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?.json::<T>().await?;
        Ok(response)
    }

    async fn post<B: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<R, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await?
            .json::<R>()
            .await?;
        Ok(response)
    }

    async fn put<B: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<R, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .put(&url)
            .json(body)
            .send()
            .await?
            .json::<R>()
            .await?;
        Ok(response)
    }

    /// Fetches an already initialized scenario
    pub async fn get_scenario(&self, scenario_id: &str) -> Result<Scenario, Box<dyn Error>> {
        let scenario: Scenario = self
            .get(&format!("/Scenarios/get_scenario/{}", scenario_id))
            .await?;
        Ok(scenario)
    }

    /// Initializes a scenario from a database scenario - this must be the first step
    pub async fn initialize_scenario(
        &self,
        db_scenario_id: &str,
    ) -> Result<Scenario, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct InitializeScenarioResponse {
            message: Option<String>,
            error: Option<String>,
            scenario: Option<Scenario>,
        }

        let resp: InitializeScenarioResponse = self
            .client
            .post(&format!(
                "{}/Scenarios/initialize_scenario?db_scenario_id={}",
                self.base_url, db_scenario_id
            ))
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?
            .json()
            .await?;

        if let Some(scenario) = resp.scenario {
            return Ok(scenario);
        }

        return Err(match resp.error {
            Some(e) => e,
            None => "No error from backend, but also no scenario, lol".to_string(),
        }
        .into());
    }

    /// Assigns vehicles to customers.
    /// Recommendation: fail if failed_to_update is not empty
    pub async fn update_scenario(
        &self,
        scenario_id: &str,
        update_vehicles: &UpdateScenario,
    ) -> Result<UpdateScenarioResponse, Box<dyn Error>> {
        let scenario: UpdateScenarioResponse = self
            .put(
                &format!("/Scenarios/update_scenario/{}", scenario_id),
                update_vehicles,
            )
            .await?;
        Ok(scenario)
    }

    /// Launches a scenario with a given speed
    pub async fn launch_scenario(
        &self,
        scenario_id: &str,
        speed: f64,
    ) -> Result<LaunchScenarioResponse, Box<dyn Error>> {
        let response = self
            .client
            .post(&format!(
                "/Runner/launch_scenario/{}?speed={}",
                scenario_id, speed
            ))
            .send()
            .await?
            .text()
            .await?;

        // Print the raw response body to stdout
        println!("Raw response: {}", response);

        // Deserialize the response body into the expected struct
        let start: LaunchScenarioResponse = serde_json::from_str(&response)?;

        Ok(start)
    }
}
