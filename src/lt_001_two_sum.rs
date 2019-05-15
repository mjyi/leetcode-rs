/*
Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

Example:
```
Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
```
 */

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            let y = target - value;

            if map.contains_key(&y) {
                let y_v = map.get(&y).unwrap();

                return vec![*y_v, index as i32];
            } else {
                map.insert(value, index as i32);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {}
