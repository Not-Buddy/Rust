impl Solution{
    pub fn my_atoi(s: String) -> i32{
        let chars: Vec<char> = s.chars().collect();
        let mut i=0;
        let n = chars.len();

        while i<n &&chars[i]==' '{
            i+=1;
        }

        let mut sign = 1;
        if i<n && (chars[i] == '+'||chars[i]=='-'){
            sign = if chars[i] == '-'{-1} else {1};
            i+=1;
        }
        
        let mut result: i64 = 0;
        while i<n && chars[i].is_ascii_digit(){
            result = result * 10 + (chars[i] as i64 - '0' as i64);
            if sign == 1 && result > i32::MAX as i64{
                return i32::MAX;
            }
            if sign == -1 && result > i32::MAX as i64+1{
                return i32::MIN;
            }
            i+=1;
        }
        (sign * result as i32)
            
    }
}
