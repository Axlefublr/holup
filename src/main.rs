use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_days(1));
    }
}

const SECONDS_IN_DAY: u64 = 60 * 60 * 24;

trait FromDaysable {
    fn from_days(days: u64) -> Self;
}

impl FromDaysable for Duration {
    fn from_days(days: u64) -> Self {
        Duration::from_secs(SECONDS_IN_DAY)
    }
}