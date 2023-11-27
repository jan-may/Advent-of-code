use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_04.txt");
    let mut valid_passphrases = 0;
    let passphrases = parse_passphrase(&input);
    for passphrase in passphrases {
        match check_passphrase(passphrase) {
            true => valid_passphrases += 1,
            false => continue
        }
    }
    valid_passphrases
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_04.txt");
    let mut valid_passphrases = 0;
    let passphrases = parse_passphrase(&input);
    for passphrase in passphrases {
        match check_anagram_passphrase(passphrase) {
            true => valid_passphrases += 1,
            false => continue
        }
    }
    valid_passphrases
}

fn check_passphrase(passphrase: Vec<String>) -> bool {
    let mut hash_set = std::collections::HashSet::new();
    for word in passphrase {
        if hash_set.contains(&word) {
            return false;
        }
        hash_set.insert(word);
    }
    true
}

fn check_anagram_passphrase(passphrase: Vec<String>) -> bool {
    let mut hash_set = std::collections::HashSet::new();
    for word in passphrase {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let word: String = chars.into_iter().collect();
        if hash_set.contains(&word) {
            return false;
        }
        hash_set.insert(word);
    }
    true
}

fn parse_passphrase(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut passphrases: Vec<Vec<String>> = Vec::new();
    for line in input {
        let mut passphrase: Vec<String> = Vec::new();
        for word in line.split_whitespace() {
            passphrase.push(word.to_string());
        }
        passphrases.push(passphrase);
    }
    passphrases
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let passphrases = parse_passphrase(&vec![
            "aa bb cc dd ee".to_string(),
            "aa bb cc dd aa".to_string(),
            "aa bb cc dd aaa".to_string()
        ]);
        assert_eq!(check_passphrase(passphrases[0].clone()), true);
        assert_eq!(check_passphrase(passphrases[1].clone()), false);
        assert_eq!(check_passphrase(passphrases[2].clone()), true);
    }

    #[test]
    fn test_part_2() {
        let passphrases = parse_passphrase(&vec![
            "abcde fghij".to_string(),
            "abcde xyz ecdab".to_string(),
            "a ab abc abd abf abj".to_string(),
            "iiii oiii ooii oooi oooo".to_string(),
            "oiii ioii iioi iiio".to_string()
        ]);
        assert_eq!(check_anagram_passphrase(passphrases[0].clone()), true);
        assert_eq!(check_anagram_passphrase(passphrases[1].clone()), false);
        assert_eq!(check_anagram_passphrase(passphrases[2].clone()), true);
        assert_eq!(check_anagram_passphrase(passphrases[3].clone()), true);
        assert_eq!(check_anagram_passphrase(passphrases[4].clone()), false);
    }
}