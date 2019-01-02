use chrono::NaiveDateTime;
use regex::Regex;
use regex::Captures;

#[derive(PartialEq,Debug)]
pub enum Record {
    GuardBeginsShift { id: u32, at: NaiveDateTime},
    GuardFallsAsleep { at: NaiveDateTime },
    GuardWakesUp { at: NaiveDateTime }
}

impl Record {

    pub fn from(record: &str) -> Option<Record> {
        lazy_static! {
            static ref GUARD_BEGINS_SHIFT: Regex = Regex::new(r"\[(?P<at>\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] Guard #(?P<guard>\d{1,}) begins shift").unwrap();
            static ref GUARD_FALL_ASLEEP: Regex = Regex::new(r"\[(?P<at>\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] falls asleep").unwrap();
            static ref GUARD_WAKES_UP: Regex = Regex::new(r"\[(?P<at>\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] wakes up").unwrap();
        }
        if GUARD_BEGINS_SHIFT.is_match(record) {
            return GUARD_BEGINS_SHIFT.captures(record).map(Record::begins_shift);
        }
        if GUARD_FALL_ASLEEP.is_match(record) {
            return GUARD_FALL_ASLEEP.captures(record).map(Record::falls_asleep);
        }
        if GUARD_WAKES_UP.is_match(record) {
            return GUARD_WAKES_UP.captures(record).map(Record::wakes_up);
        }
        None
    }


    fn begins_shift(capture: Captures) -> Record {
        Record::GuardBeginsShift {
            id: capture["guard"].parse::<u32>().unwrap(), 
            at: Record::at(&capture["at"])
        }
    }

    fn falls_asleep(capture: Captures) -> Record {
        Record::GuardFallsAsleep {
            at: Record::at(&capture["at"])
        }
    }

    fn wakes_up(capture: Captures) -> Record {
        Record::GuardWakesUp {
            at: Record::at(&capture["at"])
        }
    }

    fn at(instant: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(instant, "%Y-%m-%d %H:%M").unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::Record;

    #[test]
    fn should_return_guard_begins_shift_record() {
        assert_eq!(Record::from("[1518-10-16 00:04] Guard #479 begins shift"), Some(Record::GuardBeginsShift {
            id: 479, 
            at: Record::at("1518-10-16 00:04")
        }));
    }

    #[test]
    fn should_return_guard_falls_asleep_record() {
        assert_eq!(Record::from("[1518-07-18 00:55] falls asleep"), Some(Record::GuardFallsAsleep {
            at: Record::at("1518-07-18 00:55")
        }));
    }

    #[test]
    fn should_return_guard_wakes_up_record() {
        assert_eq!(Record::from("[1518-05-03 00:57] wakes up"), Some(Record::GuardWakesUp {
            at: Record::at("1518-05-03 00:57")
        }));
    }

    #[test]
    fn should_return_none() {
        assert_eq!(Record::from("[1518-05-03 00:57] invalid"), None);
    }
}