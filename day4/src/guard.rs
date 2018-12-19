use shift::Shift;
use std::time::Instant;
use std::time::Duration;

#[derive(Clone,Debug)]
pub struct Guard {
    id: u32,
    repose: Duration
}

impl Guard {

    pub fn new(id: u32) -> Guard {
        Guard { id: id, repose: Duration::new(0, 0) }
    }

    pub fn take(&self, shift: Shift) -> OnShiftGuard {
        OnShiftGuard { guard: self.clone(), shift: shift }
    }

}

#[derive(Clone,Debug)]
pub struct OnShiftGuard {
    guard: Guard,
    shift: Shift
}

impl OnShiftGuard {

    pub fn sleep(&self, at: Instant) -> SleepingGuard {
        SleepingGuard { on_shift: self.clone(), start: at }
    }
}

pub struct SleepingGuard {
    on_shift: OnShiftGuard,
    start: Instant
}

impl SleepingGuard {

    pub fn wake_up(&self, at: Instant) -> OnShiftGuard {
        let mut on_shift = self.on_shift.clone();
        on_shift.guard.repose += at.duration_since(self.start);
        on_shift
    }
}

#[cfg(test)]
mod tests {

    use std::time::Instant;
    use std::time::Duration;
    use shift::Shift;
    use super::Guard;

    #[test]
    fn should_have_no_repose_before_taking_shift() {
        assert_eq!(Guard::new(1).repose, Duration::new(0, 0));
    }

    #[test]
    fn should_have_no_repose_when_taking_shift() {
        let on_shift = Guard::new(1).take(a_shift());
        assert_eq!(on_shift.guard.repose, Duration::new(0, 0));
    }

    #[test]
    fn should_have_10_sec_repose() {
        let now = Instant::now();
        let on_shift = Guard::new(1).take(a_shift())
                        .sleep(now)
                        .wake_up(now + seconds(10));
        assert_eq!(on_shift.guard.repose, seconds(10));
    }

    #[test]
    fn should_have_30_sec_sleeping_time() {
        let now = Instant::now();
        let on_shift = Guard::new(1).take(a_shift())
                        .sleep(now)
                        .wake_up(now + seconds(10))
                        .sleep(now + seconds(20))
                        .wake_up(now + seconds(30))
                        .sleep(now + seconds(40))
                        .wake_up(now + seconds(50));
        assert_eq!(on_shift.guard.repose, seconds(30));
    }

    fn a_shift() -> Shift {
        Shift::new(Instant::now())
    }

    fn seconds(seconds: u64) -> Duration {
        Duration::new(seconds, 0)
    }
}