// Problem 27 - Remove Element
// https://leetcode.com/problems/remove-element/

// TODO Refactor this to use a struct with a method remove_element(val: i32)

pub struct Solution;

impl Solution {
    // Remove all occurrences of val in nums in-place
    // Return k after placing the final result in the first k slots of nums
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        return Solution::_remove_element(nums, val, 0) as i32;
    }

    // Solves array for nums[index..]
    // Returns starting index of the discarded part of the Vec
    fn _remove_element(nums: &mut Vec<i32>, val: i32, index: usize) -> usize {
        // Base case
        if index >= nums.len() {
            return nums.len();
        }

        if nums[index] == val {
            // Solve for the rest of the array
            // Find (and return) a valid swap_index for this index
            let swap_index = Solution::_remove_element(nums, val, index + 1);
            nums.swap(index, swap_index - 1);
            return swap_index - 1;
        }

        return Solution::_remove_element(nums, val, index + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums: Vec<i32> = vec![3, 2, 2, 3];
        let mut val: i32 = 3;
        let mut expected = 2;
        let mut actual = Solution::remove_element(&mut nums, val);
        assert_eq!(expected, actual);

        nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        val = 2;
        expected = 5;
        actual = Solution::remove_element(&mut nums, val);
        assert_eq!(expected, actual);
    }
}
