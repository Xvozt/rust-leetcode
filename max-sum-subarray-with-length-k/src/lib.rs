// Дан массив чисел nums и целое k.
// Найди максимальную сумму подмассива длиной k.
// Пример:
// nums := []int{1, 4, 2, 10, 23, 3, 1, 0, 20}
// k := 4
// Результат:
// 39

pub fn find_max_subarray_sum(nums: &[i32], k: usize) -> Option<i32> {
    if nums.is_empty() || k == 0 || k > nums.len() {
        return None;
    }

    let mut sum = 0;
    for i in 0..k {
        sum += nums[i];
    }

    let mut max_sum = sum;
    
    for i in k..nums.len() {
        sum = sum - nums[i-k] + nums[i];
        if sum > max_sum {
            max_sum = sum;
        }
    }
    return Some(max_sum);
    
}

pub fn find_max_subarray_sum_idiomatic(nums: &[i32], k: usize) -> Option<i32> {
    if nums.is_empty() || k == 0 || k > nums.len() {
        return None;
    }

    let mut current_sum: i32 = nums.iter().take(k).sum();
    let mut max_sum = current_sum;

    for (out, in_elem) in nums.iter().zip(nums.iter().skip(k)) {
        current_sum = current_sum - out + in_elem;
        max_sum = max_sum.max(current_sum);
    }

    Some(max_sum)

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = [1, 4, 2, 10, 23, 3, 1, 0, 20];
        let result = find_max_subarray_sum(&nums, 4);
        assert_eq!(result.unwrap(), 39);
    }
    
    #[test]
    fn it_works_idiomatic() {
        let nums = [1, 4, 2, 10, 23, 3, 1, 0, 20];
        let result = find_max_subarray_sum_idiomatic(&nums, 4);
        assert_eq!(result.unwrap(), 39);
    }
}
