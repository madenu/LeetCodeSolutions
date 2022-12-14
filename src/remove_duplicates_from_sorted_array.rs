// Problem 26 - Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut leader_index;
        let mut follower_index = 0;
        for index in 0..nums.len() {
            leader_index = index;
            if nums[leader_index] == nums[follower_index] {
                continue;
            }
            follower_index += 1;
            nums[follower_index] = nums[leader_index]
        }

        // println!("{:?}", nums);
        return (follower_index + 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = vec![1, 1, 2];
        let mut result = Solution::remove_duplicates(&mut input);
        assert_eq!(2, result);

        input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        result = Solution::remove_duplicates(&mut input);
        assert_eq!(5, result);
    }
}
