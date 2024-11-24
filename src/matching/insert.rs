use crate::matching::{Solution, COST_FUNCTION};
use crate::models::{Customer, Vehicle};

pub fn greedy(vehicles: &Vec<Vehicle>, solution: &mut Solution, removed: Vec<Customer>) {
    for customer in removed {
        let mut best_idx = (0, 0);
        let mut min_increase = f64::MAX;
        let mut routecosts = vec![];
        for i in 0..solution.route.len() {
            let mut cur = 0.0;
            //check if insert at start
            let old = if solution.route.len() != 0 {
                COST_FUNCTION(
                    vehicles[i].coord_x,
                    vehicles[i].coord_y,
                    solution.route[i][0].coord_x,
                    solution.route[i][0].coord_y,
                )
            } else {
                0.0
            };
            let new = if solution.route.len() != 0 {
                COST_FUNCTION(
                    vehicles[i].coord_x,
                    vehicles[i].coord_y,
                    customer.coord_x,
                    customer.coord_y,
                ) + COST_FUNCTION(
                    customer.destination_x.unwrap(),
                    customer.destination_y.unwrap(),
                    solution.route[i][0].coord_x,
                    solution.route[i][0].coord_y,
                )
            } else {
                COST_FUNCTION(
                    vehicles[i].coord_x,
                    vehicles[i].coord_y,
                    customer.coord_x,
                    customer.coord_y,
                )
            };
            if new - old < min_increase {
                best_idx = (i, 0);
                min_increase = new - old;
            }
            cur += old;
            for j in 0..solution.route.len() - 1 {
                cur += COST_FUNCTION(
                    solution.route[i][j].coord_x,
                    solution.route[i][j].coord_y,
                    solution.route[i][j].destination_x.unwrap(),
                    solution.route[i][j].destination_y.unwrap(),
                );
                let old = COST_FUNCTION(
                    solution.route[i][j].destination_x.unwrap(),
                    solution.route[i][j].destination_y.unwrap(),
                    solution.route[i][j + 1].coord_x,
                    solution.route[i][j + 1].coord_y,
                );
                let new = COST_FUNCTION(
                    solution.route[i][j].destination_x.unwrap(),
                    solution.route[i][j].destination_y.unwrap(),
                    customer.coord_x,
                    customer.coord_y,
                ) + COST_FUNCTION(
                    customer.destination_x.unwrap(),
                    customer.destination_y.unwrap(),
                    solution.route[i][j + 1].coord_x,
                    solution.route[i][j + 1].coord_y,
                );
                if (new - old < min_increase) {
                    best_idx = (i, j);
                    min_increase = new - old;
                }
                cur += new;
            }
            let last = solution.route[i].len() - 1;
            cur += COST_FUNCTION(
                solution.route[i][last].coord_x,
                solution.route[i][last].coord_y,
                solution.route[i][last].destination_x.unwrap(),
                solution.route[i][last].destination_y.unwrap(),
            );
            //test insert after last
            let new = COST_FUNCTION(
                solution.route[i][last].destination_x.unwrap(),
                solution.route[i][last].destination_y.unwrap(),
                customer.coord_x,
                customer.coord_y,
            );
            if new < min_increase {
                best_idx = (i, last + 1);
                min_increase = new;
            }
            routecosts.push(cur);
        }
        solution
            .route
            .get_mut(best_idx.0)
            .unwrap()
            .insert(best_idx.1, customer);
    }
}
