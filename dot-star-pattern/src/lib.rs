struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        is_match(s.as_bytes(), p.as_bytes())
    }
}

fn is_match(s: &[u8], p: &[u8]) -> bool {
    match parse(p) {
        (Pattern::Empty, _) => s.is_empty(), // pattern is exhausted; return true if string is also exhausted
        (Pattern::Single(c), subp) => is_match_single(s, c, subp),
        (Pattern::Repeatable(c), subp) => is_match_single(s, c, p) || is_match(s, subp),
    }
}

fn is_match_single(s: &[u8], to_match: u8, p: &[u8]) -> bool {
    match s.split_first() {
        Some((c, s)) if to_match == b'.' || to_match == *c => is_match(s, p),
        _ => false,
    }
}

/// Returns the parsed pattern and the next pattern to parse.
fn parse(p: &[u8]) -> (Pattern, &[u8]) {
    match p.split_first() {
        None => (Pattern::Empty, p), // pattern is exhausted
        Some((c, p)) => match p.split_first() {
            // not empty, we split on first again
            Some((b'*', p)) => (Pattern::Repeatable(*c), p), // if a* then we return Repeatable(a) and the rest of the pattern
            _ => (Pattern::Single(*c), p),
        },
    }
}

// Parser part:

enum Pattern {
    Empty,
    Single(u8),
    Repeatable(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    // #[test]
    // fn test_dot_star_2() {
    //     assert_eq!(
    //         Solution::is_match("aa".to_string(), "a*".to_string()),
    //         false
    //     );
    // }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
    }
}
