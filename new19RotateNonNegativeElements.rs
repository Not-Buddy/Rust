use std::io::{self, Read};

struct Solution;

impl Solution {
    fn rotate_elements(nums: &mut Vec<i32>, mut k: usize) -> Vec<i32>
    {
        let ok = |x: i32| x >= 0;

        let mut v = Vec::with_capacity(nums.len());

        for &x in nums.iter(){
            if ok(x){
                v.push(x);
            }
        }

        let m = v.len();

        if m > 0 {
            k %= m;
            v.rotate_left(k);

            let mut j = 0;

            for x in nums.iter_mut(){
                if ok(*x){
                    *x = v[j];
                    j+=1;
                }
            }
        }

        nums.clone()
    }
}

fn main()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let k: usize = it.next().and_then(|x| x.parse().ok()).unwrap_or(0);

    let mut nums: Vec<i32> = it.take(n).filter_map(|x| x.parse().ok()).collect();

    let result = Solution::rotate_elements(&mut nums, k);

    for (i, val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}
