pub fn min_starting_index(s: &str, pattern: &str) -> i32 {
    let s_len = s.len();
    let p_len = pattern.len();
    
    // If the pattern is longer than the string, no match is possible
    if p_len > s_len {
        return -1;
    }
    
    // Convert strings to character vectors for easier indexing
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = pattern.chars().collect();
    
    // Iterate over all possible starting positions in s
    for i in 0..=(s_len - p_len) {
        let mut diff_count = 0;
        
        // Compare the substring with the pattern
        for j in 0..p_len {
            if s_chars[i + j] != p_chars[j] {
                diff_count += 1;
                if diff_count > 1 {
                    break;
                }
            }
        }
        
        // If the difference count is at most 1, return the current index
        if diff_count <= 1 {
            return i as i32;
        }
    }
    
    // If no match is found, return -1
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert_eq!(min_starting_index("hello", "hello"), 0);
    }

    #[test]
    fn test_no_match() {
        assert_eq!(min_starting_index("hello", "world"), -1);
    }

    #[test]
    fn test_pattern_longer_than_string() {
        assert_eq!(min_starting_index("hi", "hello"), -1);
    }

    #[test]
    fn test_empty_string_or_pattern() {
        assert_eq!(min_starting_index("", ""), 0);
        assert_eq!(min_starting_index("hello", ""), 0);
        assert_eq!(min_starting_index("", "hello"), -1);
    }
}