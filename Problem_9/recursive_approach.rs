impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0 || (x%10==0 && x!=0) {return false};
        Self::recursive_approach(x,0)
    }
    pub fn recursive_approach(given: i32, reverse: i32) -> bool{
        if given>reverse {
            return Self::recursive_approach(given/10,reverse*10+(given%10))
        } else{
            return given==reverse || given==reverse/10;
        }
    }
}
