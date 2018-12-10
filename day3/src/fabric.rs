use claim::Claim;

pub struct Fabric {
    area: [[char; 1000]; 1000]
}

impl Fabric {

    pub fn new() -> Fabric {
        Fabric { area: [['.'; 1000]; 1000] }
    }

    pub fn claim(&self, claim: Claim) {
        self.area[claim.area.start.x][claim.area.start.y] = claim.id;
    }
}