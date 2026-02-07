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
            let mut stack_len = stack.len();
            let mut back = stack[stack_len - 1];
            let mut num = nums_vec[back];

            let mut potential_min = Some(stack_len);
            let mut needs_pop = false;
            //println!("Stack: {:?}", stack);
            for (ct, goal) in goal_vec.iter_mut().enumerate() {
                let old_value = *goal;
                let new_value = old_value - ((num >> ct) & 1);
                *goal = new_value;
                //println!("{}: {} -> {}  ", ct, old_value, new_value);


                if new_value != 0 {
                    potential_min = None;

                    if new_value < 0 {
                        needs_pop = true;
                    }
                }
            }
            //println!();
            
            if potential_min.is_some() {
                if min.is_none() || potential_min.unwrap() < min.unwrap() {
                    min = potential_min;
                }
            }

            if needs_pop {
                while stack.len() > 0 {
                    stack_len = stack.len();
                    back = stack[stack_len - 1];
                    num = nums_vec[back];

                    for (ct, goal) in goal_vec.iter_mut().enumerate() {
                        *goal += (num >> ct) & 1;
                    }
                    
                    if back < nums_vec.len() - 1 {
                        stack[stack_len - 1] = back + 1;
                        break;
                    }
                    else{
                        stack.pop();
                    }
                }   
            }
            else {
                stack.push(back);
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
                goal_vec.push(num);
            }

            let result = sum_slice(&mut goal_vec, &nums_vec).unwrap_or(19090909);
            total += result;
            println!("Result: {}", result);
        }
    }
    
    return total;
}
fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    let result = problem_two("input.txt");
    println!("{}",result);
}
