pub enum Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }
        
        let mid = num / 2;
        for i in 1..mid + 1 {
            if i.pow(2) == num {
                return true; 
            }
        }
        false
    }
}
