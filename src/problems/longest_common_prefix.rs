/*
Write a function to find the longest common prefix string amongst an array of strings.
If there is no common prefix, return an empty string "".
*/

pub struct TestCase {
    pub input: Vec<String>,
    pub expected: String,
}

pub fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ],
            expected: String::from("fl"),
        },
        TestCase {
            input: vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car"),
            ],
            expected: String::from(""),
        },
        TestCase {
            input: vec![
                String::from("apple"),
                String::from("app"),
                String::from("apricot"),
            ],
            expected: String::from("ap"),
        },
    ]
}

pub fn solution(strs: &[String]) -> String {
    let mut res = strs[0].clone();

    for word in strs.iter().skip(1) {
        let mut i = 0;
        let res_bytes = res.as_bytes();
        let word_bytes = word.as_bytes();

        while i < res_bytes.len() && i < word_bytes.len() && word_bytes[i] == res_bytes[i] {
            i += 1;
        }
        res = res[..i].to_string();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        for case in get_test_cases() {
            let result = solution(&case.input);
            assert_eq!(result, case.expected);
        }
    }
}