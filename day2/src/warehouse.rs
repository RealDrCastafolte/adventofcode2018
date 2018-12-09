extern crate edit_distance;

use package::Package;

#[derive(Debug)]
pub struct Warehouse {
    packages: Vec<Package>
}

impl Warehouse {

    pub fn new(packages: Vec<Package>) -> Warehouse {
        Warehouse {
            packages: packages
        }
    }

    pub fn checksum(&self) -> u32 {
        let letter_appears_two_times = self.packages.iter().fold(0, |count, b| count + b.letter_appears_two_times() as u32);
        let letter_appears_three_times = self.packages.iter().fold(0, |count, b| count + b.letter_appears_three_times() as u32);
        letter_appears_two_times * letter_appears_three_times
    }

    pub fn prototypes(&self) -> Option<String> {
        let sorted_package_ids = self.sorted_package_ids();
        let mut prototype_sample: Option<(&str, &str)> = None;
        for i in 0..sorted_package_ids.len()-1 {
            let current_package = sorted_package_ids.get(i).unwrap();
            let next_package = sorted_package_ids.get(i+1).unwrap();
            if self.is_distance_1(current_package, next_package) {
                prototype_sample = Some((current_package, next_package));
                break;
            }
        }
        prototype_sample.map(|p| {
            let mut prototype = String::new();
            for i in 0..p.0.len() {
                if p.0.chars().nth(i).eq(&p.1.chars().nth(i)) {
                    prototype.push(p.0.chars().nth(i).unwrap());
                }
            }
            prototype
        })
    }

    fn sorted_package_ids(&self) -> Vec<String> {
        let mut sorted_packages = self.packages.iter().map(|p| p.id()).collect::<Vec<String>>();
        sorted_packages.sort();
        sorted_packages
    }

    fn is_distance_1(&self, first: &str, second: &str) -> bool {
        edit_distance::edit_distance(first, second) == 1
    }
}

#[cfg(test)]
mod tests {

    use package::Package;
    use super::Warehouse;

    #[test]
    fn should_have_checksum_of_12() {
        let packages = packages(&["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]);
        let warehouse = Warehouse::new(packages);
        assert_eq!(warehouse.checksum(), 12);
    }

    #[test]
    fn should_have_packages_fgij_as_prototypes() {
        let packages = packages(&["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]);
        let warehouse = Warehouse::new(packages);
        assert_eq!(warehouse.prototypes(), Some(String::from("fgij")));
    }

    fn packages(ids: &[&str]) -> Vec<Package> {
        ids.iter().map(|id| Package::new(id.to_string())).collect()
    }
}