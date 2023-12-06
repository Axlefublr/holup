use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_days(1));
    }
}

const SECONDS_IN_DAY: u64 = 60 * 60 * 24;

/*
 * So the reason we have to do it this way is hilarious
 * You can't just impl From<u64> for Duration because you're implementing
 * an external trait for a external type
 * The orphan rules state that you have to have at least one of those be local
 * This means that if we make *our own* trait and implement *that*, even if it ends up
 * being the same exact thing, it's now suddenly allowed
 * Silly if you ask me
 */
trait FromDaysable {
    fn from_days(days: u64) -> Self;
}

impl FromDaysable for Duration {
    fn from_days(days: u64) -> Self {
        Duration::from_secs(SECONDS_IN_DAY * days)
    }
}
