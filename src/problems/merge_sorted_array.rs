/*
You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
Merge nums1 and nums2 into a single array sorted in non-decreasing order.
The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
*/

pub fn solution(nums1: &mut Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {

    let mut p = (nums1.len() - 1) as isize;
    let mut n1 = m-1;
    let mut n2 = n-1;
    
    while n1 >= 0 && n2 >= 0 {
        if nums1[n1 as usize] >= nums2[n2  as usize] {
            nums1[p as usize] = nums1[n1 as usize];
            n1 -= 1;
        } else {
            nums1[p as usize] = nums2[n2 as usize];
            n2 -= 1;
        }

        p -= 1;
    }

    while n2 >= 0 {
        nums1[p as usize] = nums2[n2 as usize];
        p -= 1;
        n2 -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;


    struct Input {
        nums1: Vec<i32>,
        m: i32,
        nums2: Vec<i32>,
        n: i32
    }
    
    struct TestCase {
        input: Input,
        expected: Vec<i32>
    }
    
    fn get_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                input: Input {
                    nums1: vec![1,2,3,0,0,0],
                    m: 3,
                    nums2: vec![2,5,6],
                    n: 3
                },
                expected: vec![1,2,2,3,5,6]
            },
            TestCase {
                input: Input { 
                    nums1: vec![1], 
                    m: 1,
                    nums2: vec![],
                    n: 0 },
                expected: vec![1]
            },
            TestCase {
                input: Input { 
                    nums1: vec![0], 
                    m: 0,
                    nums2: vec![1],
                    n: 1 },
                expected: vec![1]
            }
        ]
    }
    
    
    #[test]
    fn test_merge_sorted_array() {
        for mut case in get_test_cases() {
            solution(&mut case.input.nums1, case.input.m, case.input.nums2, case.input.n);
            assert_eq!(case.input.nums1, case.expected);
        }

        
    }
}


