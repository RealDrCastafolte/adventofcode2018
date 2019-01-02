use std::time::Duration;
use chrono::NaiveDateTime;
use shift::Shift;

#[derive(PartialEq,Clone,Debug)]
pub struct Guard {
    id: u32,
    repose: Duration
}

impl Guard {

    pub fn new(id: u32) -> Guard {
        Guard { id: id, repose: Duration::new(0, 0) }
    }

    pub fn take(&self, shift: Shift) -> OnShiftGuard {
        OnShiftGuard { on_duty: self.clone(), shift: shift }
    }

}

#[derive(PartialEq,Clone,Debug)]
pub struct OnShiftGuard {
    on_duty: Guard,
    shift: Shift
}

impl OnShiftGuard {

    pub fn sleep(&self, at: NaiveDateTime) -> SleepingGuard {
        SleepingGuard { on_shift: self.clone(), start: at }
    }
}

#[derive(PartialEq,Debug)]
pub struct SleepingGuard {
    on_shift: OnShiftGuard,
    start: NaiveDateTime
}

impl SleepingGuard {

    pub fn wake_up(&self, at: NaiveDateTime) -> OnShiftGuard {
        let mut on_shift = self.on_shift.clone();
        on_shift.on_duty.repose += (at - self.start).to_std().unwrap();
        on_shift
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDateTime;
    use std::time::Duration;
    use shift::Shift;
    use super::Guard;

    #[test]
    fn should_have_no_repose_before_taking_shift() {
        assert_eq!(Guard::new(1).repose, Duration::new(0, 0));
    }

    #[test]
    fn should_have_no_repose_when_taking_shift() {
        let on_shift = Guard::new(1).take(a_shift(at_christmas()));
        assert_eq!(on_shift.on_duty.repose, Duration::new(0, 0));
    }

    #[test]
    fn should_have_10_sec_repose() {
        let christmas = at_christmas();
        let on_shift = Guard::new(1).take(a_shift(at_christmas()))
                        .sleep(christmas)
                        .wake_up(christmas + seconds(10));
        assert_eq!(on_shift.on_duty.repose, std_seconds(10));
    }

    #[test]
    fn should_have_30_sec_sleeping_time() {
        let christmas = at_christmas();
        let on_shift = Guard::new(1).take(a_shift(at_christmas()))
                        .sleep(christmas)
                        .wake_up(christmas + seconds(10))
                        .sleep(christmas + seconds(20))
                        .wake_up(christmas + seconds(30))
                        .sleep(christmas + seconds(40))
                        .wake_up(christmas + seconds(50));
        assert_eq!(on_shift.on_duty.repose, std_seconds(30));
    }

    fn at_christmas() -> NaiveDateTime {
        NaiveDateTime::parse_from_str("2018-12-25 00:00", "%Y-%m-%d %H:%M").unwrap()
    }

    fn a_shift(at: NaiveDateTime) -> Shift {
        Shift::new(at)
    }

    fn seconds(seconds: i64) -> chrono::Duration {
        chrono::Duration::seconds(seconds)
    }

    fn std_seconds(seconds: u64) -> std::time::Duration {
        std::time::Duration::from_secs(seconds)
    }
}