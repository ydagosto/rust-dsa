pub fn solution(nums: &mut Vec<i32>, val: i32) -> i32 {

    if nums.is_empty() {
        return 0;
    }
    
    let mut endp = nums.len() - 1;
    let mut startp = 0;

    while startp < endp {
        if nums[startp] == val {
            nums[startp] = nums[endp];
            endp -= 1;
        } else {
            startp += 1
        }
    }

    if nums[endp] == val {
        return endp as i32;
    }
    (endp+1) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    struct Input {
        nums: Vec<i32>,
        val: i32
    }

    struct TestCase {
        input: Input,
        expected: Vec<i32>
    }

    fn generated_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                input: Input{nums: vec![3,2,2,3], val:3},
                expected: vec![2,2]
            },
            TestCase {
                input: Input{nums: vec![3,1,4,3], val:5},
                expected: vec![3,1,4,3]
            },
            TestCase {
                input: Input{nums: vec![0,1,2,2,3,0,4,2], val:2},
                expected: vec![0,1,4,0,3]
            },
            TestCase {
                input: Input { nums: vec![], val: 0 },
                expected: vec![]
            },
            TestCase {
                input: Input {nums: vec![1], val: 1},
                expected: vec![]
            },
            TestCase {
                input: Input {nums: vec![3], val: 2},
                expected: vec![3]
            }
        ]
    }

    #[test]
    fn test_remove_elements() {
        for mut case in generated_test_cases() {
            let result = solution(&mut case.input.nums, case.input.val);
            
            let mut actual: Vec<i32> = case.input.nums[..result as usize].to_vec();
            let mut expected = case.expected.clone();
    
            actual.sort();
            expected.sort();
    
            assert_eq!(actual, expected);
        }
    }

}