/*


*/

pub struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let length: usize = piles.len();
        let mut start: usize = 0;
        let mut end: usize = length - 1;
        let mut totAlice: i32 = 0;
        let mut totBob: i32 = 0;
        let mut idx: usize = 0;
        // [3, 7, 2, 3]
        // start ..... end
        while start < end {
            println!("piles[start] :: {}", piles[start]);
            println!("piles[end] :: {}", piles[end]);
            // To Calculate whose's turn
            if idx % 2 == 0 {
                // Alice
                if piles[start] > piles[end] {
                    totAlice += piles[start]; 
                    start+=1;
                } else {
                    totAlice += piles[end]; 
                    end -= 1;
                }
                
            } else {
                // Bob
                if piles[start] > piles[end] {
                    totBob += piles[start]; 
                    start+=1;
                } else {
                    totBob += piles[end]; 
                    end-=1;
                }
            }
            idx+=1;
        }
        totBob += piles[end];
        println!("{}", totAlice);
        println!("{}", totBob);
        if totAlice > totBob {
            return true;
        } else {
            return false;
        }
    }
}