/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.
 */

 pub fn solution(s: String) -> bool {
    
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '('|'['|'{' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase {
        input: String,
        expected: bool
    }

    fn get_test_cases() -> Vec<TestCase> {
        vec!{
            TestCase{
                input: String::from("()"),
                expected: true 
            },
            TestCase{
                input: String::from("()[]{}"),
                expected: true 
            },
            TestCase{
                input: String::from("(]"),
                expected: false 
            },
            TestCase{
                input: String::from("([])"),
                expected: true 
            }
        }
    }

    #[test]
    fn test_valid_parentheses() {
        for case in get_test_cases() {
            let result = solution(case.input);
            assert_eq!(result, case.expected);
        }
    }
}