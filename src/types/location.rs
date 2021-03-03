#[derive(Debug, Deserialize)]
pub struct Location{
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i32>,
    pub heading: Option<i32>,
    pub proximity_alert_radius: Option<i32>
}