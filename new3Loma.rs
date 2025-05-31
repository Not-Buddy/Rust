struct Solution;
impl Solution
{
    pub fn max_area(height: Vec<i32>)->i32
    {
        let mut max_area =0;
        let mut left =0;
        let mut right =height.len()-1;

        while left<right
        {
            let h=height[left].min(height[right]);
            let w=(right-left) as i32;
            max_area=max_area.max(h*w);
            if height[left]<height[right]
            {
                left+=1;
            }
            else 
            {
                right-=1;
            }
        }
        max_area
    }
}
fn main()
{

    let heights = vec![1,8,6,2,5,4,8,3,7];

    let result=Solution::max_area(heights);

    println!("The maximum area is :{}",result);

}
