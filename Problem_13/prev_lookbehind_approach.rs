impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn value(c: char) -> i32{
            match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _   => 0,
        }
        }
        let mut prev = 0;
        let mut total = 0;
        for c in s.chars(){
            let current = value(c);
            total += current;
            if prev < current {
                total -= 2*prev;
            }
            prev = current
        }
        total
    }
}
