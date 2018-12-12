extern crate regex;

use point::Point;
use rectangle::Rectangle;

#[derive(PartialEq, Debug)]
pub struct Claim {
    pub id: u32,
    pub area: Rectangle
}

impl Claim {

    pub fn new(id: u32, area: Rectangle) -> Claim {
        Claim { id: id, area: area }
    }

    pub fn from(claim: &str) -> Option<Claim> {
        let pattern = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        pattern.captures(claim).map(|capture| {
            let id = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let area_start_x = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let area_start_y = capture.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let area_width = capture.get(4).unwrap().as_str().parse::<u32>().unwrap();
            let area_height = capture.get(5).unwrap().as_str().parse::<u32>().unwrap();
            Claim { id: id, area: Rectangle::new(Point::new(area_start_x, area_start_y), area_width, area_height) }
        })
    }
}

#[cfg(test)]
mod tests {

    use point::Point;
    use rectangle::Rectangle;
    use super::Claim;

    #[test]
    fn should_build_a_4x4_claim_starting_from_1_3_with_id_1() {
        let claim = Claim::from("#1 @ 1,3: 4x4");
        assert_eq!(claim, Some(Claim::new(1, Rectangle::new(Point::new(1,3), 4, 4))));
    }

    #[test]
    fn should_build_a_4x4_claim_starting_from_3_1_with_id_2() {
        let claim = Claim::from("#2 @ 3,1: 4x4");
        assert_eq!(claim, Some(Claim::new(2, Rectangle::new(Point::new(3,1), 4, 4))));
    }

    #[test]
    fn should_build_a_2x2_claim_starting_from_5_5_with_id_3() {
        let claim = Claim::from("#3 @ 5,5: 2x2");
        assert_eq!(claim, Some(Claim::new(3, Rectangle::new(Point::new(5,5), 2,2))));
    }

    #[test]
    fn when_invalid_claim_pattern_then_should_return_none() {
        assert_eq!(Claim::from("invalid"), None);
    }
}