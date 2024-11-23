use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::Scenario;

#[derive(Debug, Clone)]
pub struct BackendClient {
    client: Client,
    base_url: String,
}

impl BackendClient {
    pub fn new(runner_server_base_url: &str) -> Self {
        Self {
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

    pub async fn create_scenario(
        &self,
        num_vehicles: u64,
        num_customers: u64,
    ) -> Result<Scenario, Box<dyn Error>> {
        Ok(self
            .post(
                &format!(
                    "scenario/create?numberOfVehicles={}&numberOfCustomers={}",
                    num_vehicles, num_customers
                ),
                &(),
            )
            .await?)
    }
}
