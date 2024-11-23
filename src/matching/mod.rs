mod time_functions;
mod cost_functions;
pub mod metric;
use crate::models::{Customer, Scenario, UpdateScenario, UpdateVehicle, Vehicle};
use crate::matching::cost_functions::StraightLineDistance;
use crate::matching::metric::Metric;
use crate::matching::time_functions::StraightLineTime;

const COST_FUNCTION: fn(f64, f64, f64, f64) ->f64=StraightLineDistance::calculate;
const TIME_FUNCTION: fn(f64, f64, f64, f64) ->f64=StraightLineTime::calculate;
struct Solution {
    route:Vec<Vec<Customer>>
}

pub fn compute_assignment(scenario:&Scenario) -> UpdateScenario {
    let v:Vec<Vehicle> = scenario.vehicles.iter().filter(|x| x.is_available).map(|x| x.to_owned()).collect();
    let c:Vec<Customer> = scenario.customers.iter().filter(|x| x.awaiting_service).map(|x| x.to_owned()).collect();
    let initial = construct_initial_solution(&v,&c);
    let mut updates = UpdateScenario{
        vehicles: vec![],
    };
    for (idx,vehicle) in v.iter().enumerate(){
        let update = UpdateVehicle{
            id: vehicle.id.clone(),
            customer_id: initial.route.get(idx).unwrap().iter().map(|x| x.id.to_owned()).collect(),
        };
        updates.vehicles.push(update);
    }
    updates
}

fn construct_initial_solution(vehicles:&Vec<Vehicle>, customers:&Vec<Customer>) -> Solution{
    let mut remaining = customers.clone();
    let mut current_id =0;
    let mut s = Solution {
        route: vec![]
    };
    for _vehicle in vehicles {
        s.route.push(vec![]);
    }
    let mut x_vehicle;
    let mut y_vehicle;
    while !remaining.is_empty() {
        let vehicle = vehicles.get(current_id).unwrap();
        if s.route.get(current_id).unwrap().is_empty() {
            x_vehicle=vehicle.coord_x;
            y_vehicle=vehicle.coord_y;
        } else {
            let last_customer=s.route.get(current_id).unwrap().last().unwrap();
            x_vehicle=last_customer.destination_x.unwrap();
            y_vehicle=last_customer.destination_y.unwrap();
        }
        let mut best = f64::MAX;
        let mut best_idx =0;
        for (idx,customer) in remaining.iter().enumerate() {
            let cost = COST_FUNCTION(customer.coord_x,customer.coord_y,x_vehicle,y_vehicle);
            if cost<best {
                best_idx=idx;
                best=cost;
            }
        }
        s.route.get_mut(current_id).unwrap().push(remaining.get(best_idx).unwrap().clone());
        remaining.remove(best_idx);
        current_id=(current_id+1)%vehicles.len();
    }
    s
}




/*=================TESTS===============================*/

#[test]
fn test_initial_solution1() {
    let vehicles = vec![
        Vehicle{
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
        Vehicle{
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
        }
    ];
    let customers = vec![
        Customer{
            id: "c1".to_string(),
            coord_x: 0.51,
            coord_y: 0.51,
            destination_x: Some(0.33),
            destination_y: Some(-0.20),
            awaiting_service: false,
        },
        Customer{
            id: "c2".to_string(),
            coord_x: 0.30,
            coord_y: -0.30,
            destination_x: Some(0.90),
            destination_y: Some(0.90),
            awaiting_service: false,
        },
        Customer{
            id: "c3".to_string(),
            coord_x: 0.0,
            coord_y: 0.0,
            destination_x: Some(-0.5),
            destination_y: Some(-0.5),
            awaiting_service: false,
        },
        Customer{
            id: "c4".to_string(),
            coord_x: -0.6,
            coord_y: -0.6,
            destination_x: Some(0.0),
            destination_y: Some(0.0),
            awaiting_service: false,
        }
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