use shift::Shift;
use std::time::Instant;
use std::time::Duration;

#[derive(Clone,Debug)]
pub struct Guard {
    id: u32,
    shift: Shift,
    sleep: Duration
}

impl Guard {

    pub fn new(id: u32, shift: Shift) -> Guard {
        Guard { id: id, shift: shift, sleep: Duration::new(0, 0) }
    }

    pub fn sleep(&self, at: Instant) -> SleepingGuard {
        SleepingGuard { guard: self.clone(), start: at }
    }

}

pub struct SleepingGuard {
    guard: Guard,
    start: Instant
}

impl SleepingGuard {

    pub fn wake_up(&self, at: Instant) -> Guard {
        let mut awaken_guard = self.guard.clone();
        awaken_guard.sleep += at.duration_since(self.start);
        awaken_guard
    }
}

#[cfg(test)]
mod tests {

    use std::time::Instant;
    use std::time::Duration;
    use shift::Shift;
    use super::Guard;

    #[test]
    fn should_have_no_sleeping_time() {
        let guard = Guard::new(1, Shift::new(Instant::now()));
        assert_eq!(guard.sleep, Duration::new(0, 0));
    }

    #[test]
    fn should_have_10_sec_sleeping_time() {
        let sleep_instant = Instant::now();
        let guard = Guard::new(1, Shift::new(Instant::now()))
                        .sleep(sleep_instant)
                        .wake_up(sleep_instant + Duration::new(10, 0));
        assert_eq!(guard.sleep, Duration::new(10, 0));
    }

    #[test]
    fn should_have_30_sec_sleeping_time() {
        let sleep_instant = Instant::now();
        let guard = Guard::new(1, Shift::new(Instant::now()))
                        .sleep(sleep_instant)
                        .wake_up(sleep_instant + seconds(10))
                        .sleep(sleep_instant + seconds(20))
                        .wake_up(sleep_instant + seconds(30))
                        .sleep(sleep_instant + seconds(40))
                        .wake_up(sleep_instant + seconds(50));
        assert_eq!(guard.sleep, seconds(30));
    }

    fn seconds(seconds: u64) -> Duration {
        Duration::new(seconds, 0)
    }
}