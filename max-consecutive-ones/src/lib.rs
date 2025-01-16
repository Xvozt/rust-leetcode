// Given a binary array nums, return the maximum number of consecutive 1's in the array.

// Example 1:
// Input: nums = [1,1,0,1,1,1]
// Output: 3
// Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.

// Example 2:
// Input: nums = [1,0,1,1,0,1]
// Output: 2

// Constraints:
// 1 <= nums.length <= 105
// nums[i] is either 0 or 1.

#[allow(dead_code)]
fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    nums.split(|&num| num == 0) // split vector with '0' as separator, slices contain consecutive ones
        .map(|ones| ones.len() as i32) // map each slice with ones to it's length
        .max() // find max length
        .unwrap_or(0) // return max length of consecutive ones or 0 (also handle conrner cases)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(find_max_consecutive_ones(nums), 3);
    }

    #[test]
    fn example_two() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(find_max_consecutive_ones(nums), 2);
    }
}
