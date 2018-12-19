use std::time::Instant;

#[derive(Clone,Debug)]
pub struct Shift {
    start: Instant
}

impl Shift {

    pub fn new(start: Instant) -> Shift {
        Shift { start: start }
    }
}