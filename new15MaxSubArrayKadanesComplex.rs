impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (_, best) = nums.into_iter().fold((0,i32::MIN), |(sum, best), x|
        {let new_sum = (sum+x).max(0);
        (new_sum, best.max(sum+x))});
        best
    }
}
