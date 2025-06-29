/*
Дан отсортированный массив nums и число target.
Найди индекс, по которому target должен быть вставлен, чтобы порядок сохранился.

Если target уже есть — верни его индекс.
Если нет — верни индекс, куда бы ты его вставил.

Input:  nums = [1,3,5,6], target = 5
Output: 2

Input:  nums = [1,3,5,6], target = 2
Output: 1

Input:  nums = [1,3,5,6], target = 7
Output: 4
*/

pub fn binary_search(nums: Vec<i32>, target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len() - 1;


    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return mid
        }

        if nums[mid] > target {
            right = mid - 1; 

        } 
        if nums[mid] < target {
            left = mid + 1;
        }
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_positive_search() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let got = binary_search(nums, target);
        let expected = 2;

        assert_eq!(got, expected);
    }
}
