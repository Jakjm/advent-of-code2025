use std::fs::File;
use std::path::Path;
use std::io::{BufRead,BufReader};


pub fn xor_slice(goal: i64, nums: &[i64]) -> Option<i64>{
    let mut min = None;
    if goal == 0 {
        min = Some(0);
    }
    else{
        for num in nums.iter() {
            if *num == goal {
                min = Some(1); 
                break;
            }
        }
    }
    if min.is_none() {
        for (ct, num) in nums.iter().take(nums.len() - 1).enumerate() {
            let result = xor_slice(goal ^ (*num), &nums[ct+1..]);
            if result.is_some() {
                if min.is_none() || (result.unwrap() + 1) < min.unwrap()
                {
                    min = Some(result.unwrap() + 1);
                }
            }
        }
    }
    return min;
}

pub fn problem_one(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    let mut total = 0 as i64;
    for line in all_lines.iter() {
        let tokens:Vec<&str> = line.split(" ").collect();
        if tokens.len() > 1 {
            let first = tokens[0];
            let mut goal = 0 as i64;
            for (bit_count, ch) in first.chars().skip(1).enumerate() {
                if ch == '#' {
                    goal += (1 as i64) << bit_count;
                }
            }


            let mut nums_vec = Vec::<i64>::new();
            nums_vec.reserve(tokens.len() - 2);

            for token in tokens.iter().skip(1).take(tokens.len() - 2) {
                let (_,token) = token.split_at(1);
                let (token,_) = token.split_at(token.len() - 1);
                
                let nums = token.split(",");

                let mut new_num = 0 as i64;
                for num in nums.into_iter() {
                    let num = num.trim().parse::<i64>().unwrap();
                    new_num += (1 as i64) << num;
                }

                nums_vec.push(new_num);
            }

            let result = xor_slice(goal, &nums_vec).unwrap();
            total += result;
        }
    }
    
    return total;
}
fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    let result = problem_one("input.txt");
    println!("{}",result);
}
