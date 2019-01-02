use chrono::NaiveDateTime;

#[derive(PartialEq,Clone,Debug)]
pub struct Shift {
    start: NaiveDateTime
}

impl Shift {

    pub fn new(start: NaiveDateTime) -> Shift {
        Shift { start: start }
    }
}