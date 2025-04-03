/*
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters 
and removing all non-alphanumeric characters, it reads the same forward and backward. 
Alphanumeric characters include letters and numbers.
Given a string s, return true if it is a palindrome, or false otherwise.
 */

pub fn solution(s: String) -> bool {
    let norm_s: Vec<char> = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if norm_s.len() <= 1 {
        return true
    } 
   
    let mut startp = 0;
    let mut endp = (norm_s.len().saturating_sub(1)) as isize;

    while startp <= endp {
        if norm_s[startp as usize] != norm_s[endp as usize] {
            return false
        }
        startp += 1;
        endp -= 1;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase{
        input: String,
        expected: bool
    }

    fn generated_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                input: String::from("A man, a plan, a canal: Panama"),
                expected: true
            },
            TestCase {
                input: String::from("race a car"),
                expected: false
            },
            TestCase {
                input: String::from(" "),
                expected: true
            },
            TestCase {
                input: String::from("a"),
                expected: true
            },
            TestCase {
                input: String::from("a."),
                expected: true
            }
        ]
    }

    #[test]
    fn test_valid_palindrome() {
        for case in generated_test_cases() {
            let result = solution(case.input);
            assert_eq!(result, case.expected);
        }
    }
}
