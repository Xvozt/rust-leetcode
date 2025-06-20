// Дана строка s. Определи, является ли она палиндромом, игнорируя пробелы, спецсимволы и регистр
// Input: A man, a plan, a canal: Panama
// Ouput: true
// 
// Input: race a car
// Ouput: false 

pub fn palindrome_check(s: &str) -> bool {
    
    if s.len() == 0 {
        return false 
    }

    let cleaned = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();

    let mut left = 0;
    let mut right = cleaned.len() - 1;

    while left < right {
        if cleaned[left] != cleaned[right] {
            return false
        }
        left += 1;
        right -=1; 
    }

    return true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let s = "A man, a plan, a canal: Panama";
        let result = palindrome_check(s);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_not_palindrome() {
        let s = "race a car";
        let result = palindrome_check(s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_empty_string() {
        let s = "";
        let result = palindrome_check(s);
        assert_eq!(result, false);
    }

    #[test]
    fn test_string_with_one_element() {
        let s = "a";
        let result = palindrome_check(s);
        assert_eq!(result, true);
    }
}
