pub trait Metric {
    fn calculate(x_source: f64, y_source: f64, x_dest: f64, y_dest: f64) -> f64;
}
