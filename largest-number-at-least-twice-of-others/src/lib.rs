// You are given an integer array nums where the largest integer is unique.
// Determine whether the largest element in the array is
// at least twice as much as every other number in the array.
// If it is, return the index of the largest element, or return -1 otherwise.

// Example 1:
// Input: nums = [3,6,1,0]
// Output: 1
// Explanation: 6 is the largest integer.
// For every other number in the array x, 6 is at least twice as big as x.
// The index of value 6 is 1, so we return 1.

// Example 2:
// Input: nums = [1,2,3,4]
// Output: -1
// Explanation: 4 is less than twice the value of 3, so we return -1.

// Constraints:
// 2 <= nums.length <= 50
// 0 <= nums[i] <= 100
// The largest element in nums is unique.
#[allow(dead_code)]
fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut largest = i32::MIN;
    let mut largest_index = -1;

    for (i, &num) in nums.iter().enumerate() {
        if num > largest {
            largest = num;
            largest_index = i as i32;
        }
    }

    for &num in &nums {
        if num != largest && largest < 2 * num {
            return -1;
        }
    }

    largest_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let nums = vec![3, 6, 1, 0];
        assert_eq!(dominant_index(nums), 1);
    }

    #[test]
    fn test_example_two() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(dominant_index(nums), -1);
    }

    #[test]
    fn test_zero_values() {
        let nums = vec![0, 0, 0, 1];
        assert_eq!(dominant_index(nums), 3);
    }
}
