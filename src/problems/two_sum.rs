use std::collections::HashMap;

// the file itself is already the two_sum mod. No need for mod two_sum {}
pub fn solution(nums: &[i32], target: i32) -> Vec<i32> {// you cannot return a reference to a local value

    let mut memo = HashMap::new();

    for (i, &v) in nums.iter().enumerate() {
        if let Some(&j) = memo.get(&(target - v)) {
            return vec![i as i32, j as i32];
        } else {
            memo.insert(v, i);
        }
    }

    vec![]
    
}

