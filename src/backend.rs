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
    pub fn new(backend_server_base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: backend_server_base_url.trim_end_matches('/').to_string(),
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
            .error_for_status()?
            .json::<R>()
            .await?;
        Ok(response)
    }
    pub async fn create_scenario(
        &self,
        num_vehicles: u64,
        num_customers: u64,
    ) -> Result<Scenario, Box<dyn Error>> {
        let response = self
            .client
            .post(&format!(
                "{}/scenario/create?numberOfVehicles={}&numberOfCustomers={}",
                self.base_url, num_vehicles, num_customers
            ))
            .send()
            .await?
            .error_for_status()?;

        // Print the response body
        let response_body = response.text().await?;
        println!("Response body: {}", response_body);

        // Parse the response body into a Scenario
        let scenario: Scenario = serde_json::from_str(&response_body)?;
        Ok(scenario)
    }
}
