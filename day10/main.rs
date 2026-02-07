use std::fs::File;
use std::path::Path;
use std::io::{BufRead,BufReader};


pub fn xor_slice(goal: i64, nums: &[i64]) -> Option<i64>{
    let mut min = None;
    if goal == 0 {
        min = Some(0);
    }
    else {
        for (ct, num) in nums.iter().take(nums.len()).enumerate() {
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

pub fn sum_slice(goal_vec: &mut Vec<i64>, nums_vec: &[i64]) -> Option<usize> {
    let mut min = Some(0);
    for num in goal_vec.iter() {
        if *num != 0 {
            min = None;
        }
    }
    if min.is_none() {
        let mut stack = Vec::<usize>::new();
        stack.push(0);

        while stack.len() > 0 {
            let mut exhausted = false;

            let mut potential_min = Some(stack.len());
            for i in 0..goal_vec.len()
            {
                let prev = nums_vec[stack[stack.len() - 1] as usize];
                goal_vec[i] -=  (prev >> i) & 1;
                if goal_vec[i] != 0 {
                    potential_min = None;
                    if goal_vec[i] < 0 || (((prev >> i) & 1) == 0) {
                        exhausted = true;
                    }
                }
            }
            if potential_min.is_some() {
                if min.is_none() || potential_min.unwrap() < min.unwrap() {
                    min = potential_min;
                }
            }
            else if exhausted {
                while stack.len() > 0 {
                    let prev_index = stack.len() - 1;
                    let prev = stack[prev_index];
                    for i in 0..goal_vec.len()
                    {
                        goal_vec[i] += (nums_vec[prev] >> i) & 1; 
                    }
                    if prev < nums_vec.len() - 1 {
                        stack[prev_index] = prev + 1;
                        break;
                    }
                    else {
                        stack.pop();
                    }
                }
            }
        }
    }
    return min;
}


pub fn problem_two(filename: &str) -> usize{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return 0;
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    let mut total = 0 as usize;
    for line in all_lines.iter() {
        let tokens:Vec<&str> = line.split(" ").collect();
        if tokens.len() > 1 {
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

            let mut goal_vec = Vec::<i64>::new();
            goal_vec.reserve(tokens[tokens.len() - 1].len() - 2);

            let last = tokens[tokens.len() - 1];
            let (_,last) = last.split_at(1);
            let (last,_) = last.split_at(last.len() - 1);
            let nums = last.split(",");
            for num in nums.into_iter() {
                let num = num.trim().parse::<i64>().unwrap();
                goal_vec.push((1 as i64) << num);
            }

            let result = sum_slice(&mut goal_vec, &nums_vec).unwrap();
            total += result;
        }
    }
    
    return total;
}
fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    let result = problem_two("input2.txt");
    println!("{}",result);
}
