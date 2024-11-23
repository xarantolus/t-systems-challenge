use crate::matching::cost_functions::StraightLineDistance;
use crate::matching::metric::Metric;

pub struct StraightLineTime;
impl Metric for StraightLineTime {
    fn calculate(lat_source: f64, lon_source: f64, lat_dest: f64, lon_dest: f64) -> f64 {
        StraightLineDistance::calculate(lat_source, lon_source, lat_dest, lon_dest)/9.0
    }
}