use std::collections::{HashMap, HashSet};

// Given two strings s and t, determine if they are isomorphic.
// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
// All occurrences of a character must be replaced with another character while preserving the order of characters.
// No two characters may map to the same character, but a character may map to itself.
//
// Example 1:
// Input: s = "egg", t = "add"
// Output: true
// Explanation:
// The strings s and t can be made identical by:
// Mapping 'e' to 'a'.
// Mapping 'g' to 'd'.

// Example 2:
// Input: s = "foo", t = "bar"
// Output: false
// Explanation:
// The strings s and t can not be made identical as 'o' needs to be mapped to both 'a' and 'r'.

// Example 3:
// Input: s = "paper", t = "title"
// Output: true

// Constraints:
// 1 <= s.length <= 5 * 104
// t.length == s.length
// s and t consist of any valid ascii character.
#[allow(dead_code)]
fn is_isomorphic(s: String, t: String) -> bool {
    let (s, t) = (s.into_bytes(), t.into_bytes());

    let mut mapping: HashMap<u8, u8> = HashMap::with_capacity(s.len());

    for i in 0..s.len() {
        let val = mapping.entry(s[i]).or_insert(t[i]);
        if *val != t[i] {
            return false;
        }
    }

    mapping.values().collect::<HashSet<_>>().len() == mapping.keys().collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_for_egg_add() {
        assert!(is_isomorphic(String::from("egg"), String::from("add")));
    }

    #[test]
    fn false_for_foo_bar() {
        assert!(!is_isomorphic(String::from("foo"), String::from("bar")));
    }

    #[test]
    fn true_for_paper_title() {
        assert!(is_isomorphic(String::from("paper"), String::from("title")));
    }
}
