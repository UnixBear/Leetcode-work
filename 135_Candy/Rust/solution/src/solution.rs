pub struct Solution;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.is_empty() {
            return 0;
        }
        let (mut ret, mut up, mut down, mut peak) = (1,0,0,0);
        for window in ratings.windows(2){
            if let [previous, current] = window {
                if previous < current {
                    up += 1;
                    down = 0;
                    peak = up;
                    ret += 1+up;
                }
                else if previous == current {
                    peak = 0;
                    down = peak;
                    up = down;
                    ret += 1;
                }
                else {
                    up = 0;
                    down = down + 1;
                    ret += 1 + down - ((peak >= down) as i32);
                }
            }
        }
        ret
    }
}