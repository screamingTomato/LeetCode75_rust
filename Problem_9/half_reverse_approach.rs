impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if(x%10==0 && x != 0) || x<0 {
            return false;
        }
        let mut x1: i32 = x;
        let mut reverse: i32 = 0;
        while x1>reverse {
            let last = x1%10;
            reverse = reverse*10+last;
            x1 = x1/10;
        }
        x1==reverse || x1==reverse/10
    }
}
