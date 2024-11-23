use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::models::{Scenario, UpdateScenario};

#[derive(Debug)]
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

	async fn post<B: Serialize, R: for<'de> Deserialize<'de>>(&self, endpoint: &str, body: &B) -> Result<R, Box<dyn Error>> {
		let url = format!("{}/{}", self.base_url, endpoint);
		let response = self.client.post(&url).json(body).send().await?.json::<R>().await?;
		Ok(response)
	}

	pub async fn get_scenario(&self, scenario_id: &str) -> Result<Scenario, Box<dyn Error>> {
		let scenario: Scenario = self.get(&format!("/Scenarios/get_scenario/{}", scenario_id)).await?;
		Ok(scenario)
	}

	pub async fn initialize_scenario(&self, scenario_id: &str) -> Result<Scenario, Box<dyn Error>> {
		// TODO: Not sure what this returns actually
		let scenario: Scenario = self.post(&format!("/Scenarios/initialize_scenario/{}", scenario_id), &()).await?;
		Ok(scenario)
	}

	pub async fn update_scenario(&self, scenario_id: &str, updated_scenario: &UpdateScenario) -> Result<Scenario, Box<dyn Error>> {
		// TODO: Not sure what this returns actually
		let scenario: Scenario = self.post(&format!("/Scenarios/update_scenario/{}", scenario_id), &()).await?;
		Ok(scenario)
	}


	pub async fn launch_scenario(&self, scenario_id: &str, speed: f64) -> Result<Scenario, Box<dyn Error>> {
		#[derive(Serialize)]
		struct LaunchScenarioParameters<'a> {
			speed: f64,
			scenario_id: &'a str,
		}

		// TODO: Not sure what this returns actually
		let scenario: Scenario = self.post(&format!("/Runner/launch_scenario/{}", scenario_id), &LaunchScenarioParameters{
			speed,
			scenario_id,
		}).await?;
		Ok(scenario)
	}
}
