use crate::models::{Scenario, UpdateScenario};

pub fn compute_assignment(scenario: &Scenario) -> UpdateScenario {
    UpdateScenario { vehicles: vec![] }
}
