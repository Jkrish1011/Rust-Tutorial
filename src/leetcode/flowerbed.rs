pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut counter: i32 = 0;
        if flowerbed[0] == 0 {
            counter = 1;
        }
        for idx in 0..flowerbed.len() {
            if flowerbed[idx] == 1 {
                n -= (counter-1)/2;
                counter = 0;
            } else {
                counter += 1;
            }
        }
        
        n-= (counter / 2);

        if n<= 0 {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        let res: bool = Solution::can_place_flowers(vec![1,0,0,1,0,1], 1);
        assert_eq!(res, false);
    }
}