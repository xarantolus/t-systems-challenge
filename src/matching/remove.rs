use crate::matching::Solution;
use rand::{thread_rng, Rng};

pub(crate) fn shawn_heuristic(
    solution: &Solution,
    cost: fn(f64, f64, f64, f64) -> f64,
    q: usize,
) -> Vec<(usize, usize)> {
    let mut flattened: Vec<(f64, (usize, usize))> = vec![];
    for i in 0..solution.route.len() {
        for j in 0..solution.route[i].len() {
            let customer = &solution.route[i][j];
            flattened.push((
                cost(
                    customer.coord_x,
                    customer.coord_y,
                    customer.destination_x.unwrap(),
                    customer.destination_y.unwrap(),
                ),
                (i, j),
            ));
        }
    }
    let r = thread_rng().gen_range(0..flattened.len());
    let rcost = flattened[r].0;
    flattened.sort_by(|a, b| {
        ((rcost - a.0).abs() * thread_rng().gen_range(0.95..1.05))
            .partial_cmp(&(rcost - &b.0).abs())
            .unwrap()
    });
    flattened
        .iter()
        .take(q as usize)
        .map(|(i, j)| j.to_owned())
        .collect()
}
