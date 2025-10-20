impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        return Self::max_sub_array_rec(&nums, 0, nums.len() as i32 - 1);
    }

    fn max_sub_array_rec(nums: &Vec<i32>, l: i32, r: i32) -> i32 {
        if l > r {
            return i32::MIN;
        }

        let mid = (l + r) / 2;
        let mut left_sum = 0;
        let mut right_sum = 0;
        let mut curr_sum = 0;

        // Convert indices to usize for safe indexing
        let l_usize = l as usize;
        let mid_usize = mid as usize;
        let r_usize = r as usize;

        for i in (l_usize..mid_usize).rev() {
            curr_sum += nums[i];
            left_sum = left_sum.max(curr_sum);
        }

        curr_sum = 0;
        for i in (mid_usize + 1..=r_usize) {
            curr_sum += nums[i];
            right_sum = right_sum.max(curr_sum);
        }

        let left_part = if mid > 0 {
            Self::max_sub_array_rec(nums, l, mid - 1)
        } else {
            i32::MIN
        };

        let right_part = if mid + 1 <= r {
            Self::max_sub_array_rec(nums, mid + 1, r)
        } else {
            i32::MIN
        };

        return left_part.max(right_part).max(left_sum + right_sum + nums[mid_usize]);
    }
}
