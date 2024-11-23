use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::models::{Scenario, UpdateScenario, UpdateScenarioResponse};

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
            message: String,
            scenario: Scenario,
        }

        let resp: InitializeScenarioResponse = self
            .post(
                &format!("/Scenarios/initialize_scenario/{}", db_scenario_id),
                &(),
            )
            .await?;
        Ok(resp.scenario)
    }

    /// Assigns vehicles to customers.
    /// Recommendation: fail if failed_to_update is not empty
    pub async fn update_scenario(
        &self,
        scenario_id: &str,
        update_vehicles: &UpdateScenario,
    ) -> Result<UpdateScenarioResponse, Box<dyn Error>> {
        let scenario: UpdateScenarioResponse = self
            .post(&format!("/Scenarios/update_scenario/{}", scenario_id), &())
            .await?;
        Ok(scenario)
    }

    /// Launches a scenario with a given speed
    pub async fn launch_scenario(
        &self,
        scenario_id: &str,
        speed: f64,
    ) -> Result<UpdateScenarioResponse, Box<dyn Error>> {
        #[derive(Serialize)]
        struct LaunchScenarioParameters<'a> {
            speed: f64,
            scenario_id: &'a str,
        }

        let scenario: UpdateScenarioResponse = self
            .post(
                &format!("/Runner/launch_scenario/{}", scenario_id),
                &LaunchScenarioParameters { speed, scenario_id },
            )
            .await?;
        Ok(scenario)
    }
}
