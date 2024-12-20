use std::error::Error;

use reqwest::Client;
use serde::Deserialize;

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
            base_url: runner_server_base_url.trim_end_matches('/').to_string(),
        }
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
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
            .error_for_status()?
            .json()
            .await?;

        if let Some(scenario) = resp.scenario {
            return Ok(scenario);
        }

        return Err(resp
            .error
            .unwrap_or_else(|| "No error from backend, but also no scenario, lol".to_string())
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
            .client
            .put(&format!(
                "{}/Scenarios/update_scenario/{}",
                self.base_url, scenario_id
            ))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(update_vehicles)?)
            .send()
            .await?
            .error_for_status()?
            .json()
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
                "{}/Runner/launch_scenario/{}?speed={}",
                self.base_url, scenario_id, speed
            ))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        // Deserialize the response body into the expected struct
        let start: LaunchScenarioResponse = serde_json::from_str(&response)?;

        Ok(start)
    }
}
