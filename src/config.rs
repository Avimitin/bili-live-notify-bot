use chrono::Duration;

// TODO: query duration should be convert from string to chrono::Duration
#[derive(Debug)]
pub struct Config {
    pub query_duration: Duration,
    /// Rooms amount that query in once. Recommend 1-100.
    pub query_amount: usize,
}

impl Config {
    pub fn duration(&self) -> &Duration {
        &self.query_duration
    }
}
