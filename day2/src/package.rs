use std::collections::HashMap;

#[derive(Debug)]
pub struct Package {
    id: String,
    letters_count: HashMap<char, u32>
}

impl Package {

    pub fn new(id: String) -> Package {
        let mut letters_count: HashMap<char, u32> = HashMap::new();
        for c in id.chars() {
            letters_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        Package {
            id: id,
            letters_count: letters_count
        }
    }

    pub fn letter_appears_two_times(&self) -> bool {
        self.letters_count.values().any(|c| *c == 2)
    }

    pub fn letter_appears_three_times(&self) -> bool {
        self.letters_count.values().any(|c| *c == 3)
    }
}

#[cfg(test)]
mod tests {

    use super::Package;

    #[test]
    fn should_have_no_letters_appearing_two_or_three_times() {
        let package = Package::new(String::from("abcdef"));
        assert_eq!(package.letter_appears_two_times(), false);
        assert_eq!(package.letter_appears_three_times(), false);
    }

    #[test]
    fn should_have_a_letter_appearing_two_and_three_times() {
        let package = Package::new(String::from("bababc"));
        assert_eq!(package.letter_appears_two_times(), true);
        assert_eq!(package.letter_appears_three_times(), true);
    }

    #[test]
    fn should_have_a_letter_appearing_two_times() {
        let package = Package::new(String::from("abbcde"));
        assert_eq!(package.letter_appears_two_times(), true);
        assert_eq!(package.letter_appears_three_times(), false);
    }

    #[test]
    fn should_have_a_letter_appearing_three_times() {
        let package = Package::new(String::from("abcccd"));
        assert_eq!(package.letter_appears_two_times(), false);
        assert_eq!(package.letter_appears_three_times(), true);
    }

    #[test]
    fn should_have_two_letters_appearing_two_times() {
        let package = Package::new(String::from("aabcdd"));
        assert_eq!(package.letter_appears_two_times(), true);
        assert_eq!(package.letter_appears_three_times(), false);
    }

    #[test]
    fn should_have_two_letters_appearing_three_times() {
        let package = Package::new(String::from("ababab"));
        assert_eq!(package.letter_appears_two_times(), false);
        assert_eq!(package.letter_appears_three_times(), true);
    }
}