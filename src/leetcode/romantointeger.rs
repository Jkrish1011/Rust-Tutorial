use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let lookup:HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000)
        ]);
        let input: Vec<char> = s.chars().collect();

        let mut curr: &char;
        let mut next: &char;
        let mut return_value: i32 = 0;
        let mut idx: usize = 0;
        while idx < (s.len()-1) {
            curr = &input[idx];
            next = &input[idx+1];

            if let (Some(val1), Some(val2)) = (lookup.get(curr), lookup.get(next)) {
                
                if val1 > val2 {
                    return_value += val1;
                    idx+=1;
                } else if val1 < val2 {
                    return_value += (val2-val1);
                    idx+=2;
                } else {
                    return_value += (val2+val1);
                    idx+=2;
                }
            }
        }
        
        if idx != s.len() {
            curr = &input[idx];
            if let Some(v) = lookup.get(curr) {
                return_value+=v;
            }
        }

        return return_value; 
    }
}