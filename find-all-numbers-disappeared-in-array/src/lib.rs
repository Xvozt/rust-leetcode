// Given an array nums of n integers where nums[i] is in the range [1, n],
// return an array of all the integers in the range [1, n] that do not appear in nums.

// Example 1:
// Input: nums = [4,3,2,7,8,2,3,1]
// Output: [5,6]

// Example 2:
// Input: nums = [1,1]
// Output: [2]

// Constraints:
// n == nums.length
// 1 <= n <= 105
// 1 <= nums[i] <= n

#[allow(dead_code)]
pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    for i in 0..n {
        let index = (nums[i].abs() - 1) as usize;
        if nums[index] > 0 {
            nums[index] = -nums[index];
        }
    }

    let mut result = Vec::new();
    for i in 0..n {
        if nums[i] > 0 {
            result.push(i as i32 + 1);
        }
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let expected = vec![5, 6];
        assert_eq!(find_disappeared_numbers(nums), expected)
    }

    #[test]
    fn example_two() {
        let nums = vec![1, 1];
        let expected = vec![2];
        assert_eq!(find_disappeared_numbers(nums), expected)
    }
}
