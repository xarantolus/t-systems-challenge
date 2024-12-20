mod cost_functions;
pub mod insert;
pub mod metric;
pub mod remove;
mod time_functions;

use crate::matching::cost_functions::StraightLineDistance;
use crate::matching::metric::Metric;
use crate::matching::time_functions::StraightLineTime;
use crate::models::{Customer, Scenario, Vehicle};
use rand::{thread_rng, Rng};
use std::collections::{HashMap, VecDeque};

const COST_FUNCTION: fn(f64, f64, f64, f64) -> f64 = StraightLineDistance::calculate;
const TIME_FUNCTION: fn(f64, f64, f64, f64) -> f64 = StraightLineTime::calculate;
#[derive(Clone)]
struct Solution {
    route: Vec<Vec<Customer>>,
}

pub fn compute_assignment(scenario: &Scenario) -> HashMap<String, VecDeque<String>> {
    let v: Vec<Vehicle> = scenario
        .vehicles
        .iter()
        .filter(|x| x.is_available)
        .map(|x| x.to_owned())
        .collect();
    let c: Vec<Customer> = scenario
        .customers
        .iter()
        .filter(|x| x.awaiting_service)
        .map(|x| x.to_owned())
        .collect();
    let initial = construct_initial_solution(&v, &c);
    let optimal = optimize_alns(&v, initial.clone(), 0.95, 50);
    let mut map: HashMap<String, VecDeque<String>> = HashMap::new();
    for (idx, vehicle) in v.iter().enumerate() {
        let ids: VecDeque<String> = optimal.route[idx].iter().map(|x| x.id.to_owned()).collect();
        map.insert(vehicle.id.clone(), ids);
    }
    map
}

fn optimize_alns(
    vehicles: &Vec<Vehicle>,
    initial: Solution,
    cooling_factor: f64,
    max_iterations: i32,
) -> Solution {
    const REMOVAL_FACTOR: f64 = 0.2;
    let insert_heuristics: Vec<fn(&Vec<Vehicle>, &mut Solution, Vec<Customer>)> = vec![
        insert::greedy,
        insert::greedy,
        insert::greedy,
        insert::greedy,
    ];
    let insert_weights = vec![1.0, 1.0, 1.0, 1.0];
    let remove_heuristics: Vec<
        fn(&Solution, fn(f64, f64, f64, f64) -> f64, q: usize) -> Vec<(usize, usize)>,
    > = vec![
        remove::shawn_heuristic,
        remove::shawn_heuristic,
        remove::shawn_heuristic,
    ];
    let remove_weights = vec![1.0, 1.0, 1.0];
    let requests: usize = initial.route.iter().map(|x| x.len()).sum();
    let q = (requests as f64 * REMOVAL_FACTOR).floor() as usize;
    let mut current = initial.clone();
    for i in 0..max_iterations {
        let removal = select_heuristic(&remove_weights);
        let indexes: Vec<(usize, usize)> = remove_heuristics[removal](&current, COST_FUNCTION, q);
        let mut removed: Vec<Customer> = vec![];
        let mut offsets = vec![];
        for idx in 0..indexes.len() {
            offsets.push(0);
        }
        for (i, j) in indexes {
            removed.push(current.route[i][j - offsets[i]].clone());
            current.route.get_mut(i).unwrap().remove(j - offsets[i]);
            offsets[i] += 1;
        }
        let insert = select_heuristic(&insert_weights);
        insert_heuristics[insert](&vehicles, &mut current, removed);
    }
    initial
}

fn select_heuristic(weights: &Vec<f64>) -> usize {
    let sum = weights.iter().sum();
    let mut random = thread_rng().gen_range(0.0..sum);
    let mut selected: usize = 0;
    loop {
        random = random - weights.get(selected).unwrap();
        if random > 0.0 {
            break;
        }
        selected += 1;
    }
    selected
}

fn construct_initial_solution(vehicles: &Vec<Vehicle>, customers: &Vec<Customer>) -> Solution {
    let mut remaining = customers.clone();
    let mut current_id = 0;
    let mut s = Solution { route: vec![] };
    for _vehicle in vehicles {
        s.route.push(vec![]);
    }
    let mut x_vehicle;
    let mut y_vehicle;
    while !remaining.is_empty() {
        let vehicle = vehicles.get(current_id).unwrap();
        if s.route.get(current_id).unwrap().is_empty() {
            x_vehicle = vehicle.coord_x;
            y_vehicle = vehicle.coord_y;
        } else {
            let last_customer = s.route.get(current_id).unwrap().last().unwrap();
            x_vehicle = last_customer.destination_x.unwrap();
            y_vehicle = last_customer.destination_y.unwrap();
        }
        let mut best = f64::MAX;
        let mut best_idx = 0;
        for (idx, customer) in remaining.iter().enumerate() {
            let cost = COST_FUNCTION(customer.coord_x, customer.coord_y, x_vehicle, y_vehicle);
            if cost < best {
                best_idx = idx;
                best = cost;
            }
        }
        s.route
            .get_mut(current_id)
            .unwrap()
            .push(remaining.get(best_idx).unwrap().clone());
        remaining.remove(best_idx);
        current_id = (current_id + 1) % vehicles.len();
    }
    s
}

/*=================TESTS===============================*/

#[test]
fn test_initial_solution1() {
    let vehicles = vec![
        Vehicle {
            id: "v1".to_string(),
            coord_x: 0.0,
            coord_y: 0.0,
            is_available: true,
            vehicle_speed: None,
            customer_id: None,
            remaining_travel_time: Some(0.0),
            distance_travelled: Some(0.0),
            active_time: Some(0.0),
            number_of_trips: Some(0),
        },
        Vehicle {
            id: "v2".to_string(),
            coord_x: 0.5,
            coord_y: 0.5,
            is_available: false,
            vehicle_speed: None,
            customer_id: None,
            remaining_travel_time: Some(0.0),
            distance_travelled: Some(0.0),
            active_time: Some(0.0),
            number_of_trips: Some(0),
        },
    ];
    let customers = vec![
        Customer {
            id: "c1".to_string(),
            coord_x: 0.51,
            coord_y: 0.51,
            destination_x: Some(0.33),
            destination_y: Some(-0.20),
            awaiting_service: false,
        },
        Customer {
            id: "c2".to_string(),
            coord_x: 0.30,
            coord_y: -0.30,
            destination_x: Some(0.90),
            destination_y: Some(0.90),
            awaiting_service: false,
        },
        Customer {
            id: "c3".to_string(),
            coord_x: 0.0,
            coord_y: 0.0,
            destination_x: Some(-0.5),
            destination_y: Some(-0.5),
            awaiting_service: false,
        },
        Customer {
            id: "c4".to_string(),
            coord_x: -0.6,
            coord_y: -0.6,
            destination_x: Some(0.0),
            destination_y: Some(0.0),
            awaiting_service: false,
        },
    ];
    let s = construct_initial_solution(&vehicles, &customers);
    assert_eq!(s.route.len(), 2);
    assert_eq!(s.route[0].len(), 2);
    assert_eq!(s.route[1].len(), 2);
    assert_eq!(s.route[0][0].id, "c3");
    assert_eq!(s.route[0][1].id, "c4");
    assert_eq!(s.route[1][0].id, "c1");
    assert_eq!(s.route[1][1].id, "c2");
}
