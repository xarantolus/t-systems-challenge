use crate::matching::metric::Metric;

pub struct StraightLineDistance;

const EARTH_RADIUS_METERS: f64 = 6371000.0;
impl Metric for StraightLineDistance {
    fn calculate(lat_source: f64, lon_source: f64, lat_dest: f64, lon_dest: f64) -> f64 {
        let lat1_rad = degrees_to_radians(lat_source);
        let lon1_rad = degrees_to_radians(lon_source);
        let lat2_rad = degrees_to_radians(lat_dest);
        let lon2_rad = degrees_to_radians(lon_dest);

        let delta_lat = lat2_rad - lat1_rad;
        let delta_lon = lon2_rad - lon1_rad;

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_METERS * c
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
