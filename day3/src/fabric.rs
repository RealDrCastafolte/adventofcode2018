use claim::Claim;

pub struct Fabric {
    area: Vec<Vec<String>>,
    claims: Vec<Claim>
}

impl Fabric {

    pub fn new() -> Fabric {
        Fabric { 
            area: vec![vec![String::from("."); 1000]; 1000],
            claims: Vec::new()
        }
    }

    pub fn claim(&mut self, claim: Claim) {
        for x in claim.area.start.x..(claim.area.start.x + claim.area.width) {
            for y in claim.area.start.y..(claim.area.start.y + claim.area.height) {
                self.area[x as usize][y as usize] = match self.area[x as usize][y as usize].as_ref() {
                    "." => claim.id.to_string(),
                    _ => String::from("X"),
                };
            }
        }
        self.claims.push(claim);
    }

    pub fn overlapping_claim_area(&self) -> u32 {
        let mut overlap_count = 0;
        for x in 0..self.area.len() {
            for y in 0..self.area.len() {
                if self.area[x][y] == "X" {
                    overlap_count += 1;
                }
            }
        }
        overlap_count
    }

    pub fn valid_claim(&self) -> Option<&Claim> {
        self.claims.iter().filter(|claim| !self.is_claim_overlapped(claim)).next()
    }

    fn is_claim_overlapped(&self, claim: &Claim) -> bool {
        for x in claim.area.start.x..(claim.area.start.x + claim.area.width) {
            for y in claim.area.start.y..(claim.area.start.y + claim.area.height) {
                if self.area[x as usize][y as usize] != claim.id.to_string() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use point::Point;
    use rectangle::Rectangle;
    use claim::Claim;
    use super::Fabric;

    #[test]
    fn should_init_area() {
        let fabric = Fabric::new();
        for i in 0..1000 as usize {
            for j in 0..1000 as usize {
                assert_eq!(fabric.area[i][j], ".");
            }
        }
    }

    #[test]
    fn should_claim_area() {
        let mut fabric = Fabric::new();
        let claim = Claim::new(1, Rectangle::new(Point::new(5, 5), 2, 2));
        fabric.claim(claim);
        assert_eq!(fabric.area[5][5], "1");
        assert_eq!(fabric.area[5][6], "1");
        assert_eq!(fabric.area[6][5], "1");
        assert_eq!(fabric.area[6][6], "1");
    }

    #[test]
    fn should_claim_area_with_overlap() {
        let mut fabric = Fabric::new();
        fabric.claim(Claim::new(1, Rectangle::new(Point::new(1, 3), 4, 4)));
        fabric.claim(Claim::new(2, Rectangle::new(Point::new(3, 1), 4, 4)));
        assert_eq!(fabric.area[3][3], "X");
        assert_eq!(fabric.area[3][4], "X");
        assert_eq!(fabric.area[4][3], "X");
        assert_eq!(fabric.area[4][4], "X");
    }

    #[test]
    fn should_return_overlapping_claim_area() {
        let mut fabric = Fabric::new();
        fabric.claim(Claim::new(1, Rectangle::new(Point::new(1, 3), 4, 4)));
        fabric.claim(Claim::new(2, Rectangle::new(Point::new(3, 1), 4, 4)));
        assert_eq!(fabric.overlapping_claim_area(), 4);
    }

    #[test]
    fn should_not_find_a_valid_claim() {
        let mut fabric = Fabric::new();
        fabric.claim(Claim::new(1, Rectangle::new(Point::new(1, 3), 4, 4)));
        fabric.claim(Claim::new(2, Rectangle::new(Point::new(3, 1), 4, 4)));
        assert_eq!(fabric.valid_claim(), None);
    }

    #[test]
    fn should_find_a_valid_claim() {
        let mut fabric = Fabric::new();
        fabric.claim(Claim::new(1, Rectangle::new(Point::new(1, 3), 4, 4)));
        fabric.claim(Claim::new(2, Rectangle::new(Point::new(3, 1), 4, 4)));
        fabric.claim(Claim::new(3, Rectangle::new(Point::new(5, 5), 2, 2)));
        assert_eq!(fabric.valid_claim().unwrap().id, 3);
    }
}