use std::io::{self, Read};

struct Solution;

impl Solution
{
    fn minimum_prefix_length(nums: &[i32]) -> usize
    {
        let n = nums.len();
        if n <= 1{
            return 0;
        }

        let mut i : isize = (n as isize) - 2;

        while i>=0 && nums[i as usize] < nums[i as usize+1]
        {
            i-=1;
        }

        (i+1) as usize
    }
}

fn main()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let result = Solution::minimum_prefix_length(&nums);
    println!("{}",result);
}
