use std::char;
use claim::Claim;

pub struct Fabric {
    area: [[char; 1000]; 1000]
}

impl Fabric {

    pub fn new() -> Fabric {
        Fabric { 
            area: [['.'; 1000]; 1000]
        }
    }

    pub fn claim(&mut self, claim: Claim) {
        let claim_id = char::from_u32(claim.id).unwrap();
        for i in 0..claim.area.width as usize {
            for j in 0..claim.area.height as usize {
                let cell = self.area[claim.area.start.x as usize + i][claim.area.start.y as usize + j];
                if cell == '.' {
                    self.area[claim.area.start.x as usize + i][claim.area.start.y as usize + j] = claim_id;
                } else {
                    self.area[claim.area.start.x as usize + i][claim.area.start.y as usize + j] = 'X';
                }
                println!("{}", self.area[claim.area.start.x as usize + i][claim.area.start.y as usize + j]);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use point::Point;
    use rectangle::Rectangle;
    use claim::Claim;
    use super::Fabric;

    #[test]
    fn should_claim_area() {
        // let mut fabric = Fabric::new();
        // let claim = Claim::new(1, Rectangle::new(Point::new(1, 3), 2, 2));
        // fabric.claim(claim);
        // assert_eq!(fabric.area[1][3], '1');
        // assert_eq!(fabric.area[2][3], '1');
        // assert_eq!(fabric.area[1][4], '1');
        // assert_eq!(fabric.area[2][4], '1');
    }
}