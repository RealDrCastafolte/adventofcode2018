use Package;

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

}

#[cfg(test)]
mod tests {

    use package::Package;
    use super::Warehouse;

    #[test]
    fn should_have_checksum_of_12() {
        let packages = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]
            .iter()
            .map(|id| Package::new(id.to_string()))
            .collect();
        let warehouse = Warehouse::new(packages);
        assert_eq!(warehouse.checksum(), 12);
    }

}