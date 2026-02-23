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

pub fn min_buttons_for_mask(mask : i64, nums: &[i64]) -> Option<Vec<usize>> {
    let mut min_len = usize::MAX;
    let mut res = None;
    if mask == 0 {
        return Some(Vec::<usize>::new());
    }
    else{
        for (ct, num) in nums.iter().take(nums.len()).enumerate() {
            let xor = mask ^ *num;
            if xor == 0 {
                return Some(vec![ct]);
            }
            else if (xor & *num) == 0 {
                let mut sub_res = min_buttons_for_mask(xor, &nums[ct+1..]);
                if sub_res.is_some() && 1 + sub_res.as_ref().unwrap().len() < min_len {
                    min_len = 1 + sub_res.as_ref().unwrap().len();
                    sub_res.as_mut().unwrap().push(ct);
                    res = Some(sub_res);
                }
            }
        }
    }
    if min_len != usize::MAX {
        return res?;
    }
    else {
        return None;
    }
}

pub fn sum_slice3(goal_vec: &mut Vec<i64>, nums: &[i64]) -> Option<usize>{
    let mut min = Some(0);
    for num in goal_vec.iter() {
        if *num != 0 {
            min = None;
        }
    }
    if min.is_some() {
        return min;
    }

    let mut zero = false;
    let mut multiplier = 1;
    let mut total = 0;
    while !zero{
        let mut odd_mask = 0 as i64;
        zero = true;
        for(count, num) in goal_vec.iter().enumerate(){
            if *num != 0 {
                zero = false;
                if *num & 1 == 1 {
                    odd_mask += (1 as i64) << count;
                }
            }
        }
        total += min_buttons_for_mask(odd_mask, nums).unwrap().len() * multiplier;
        for num in goal_vec.iter_mut() {
            if *num != 0 {
                *num /= 2;
            }
        }
        multiplier *= 2;
    }
    return Some(total);
}

//This approach won't work...
pub fn sum_slice2(goal_vec: &mut Vec<i64>, nums: &[i64]) -> Option<usize> {
    let mut min = Some(0);
    for num in goal_vec.iter() {
        if *num != 0 {
            min = None;
        }
    }
    if min.is_some() {
        return min;
    }

    let mut list = Vec::<usize>::new();
    for _ in 0..nums.len(){
        list.push(0);
    }
    //New approach: While goal_vec is not all zeros, find the minimum cell among those that are non-zero
    //And use an optimal number of presses to reduce all cells by that are non-zero by that amount.
    let mut zero = false;
    while !zero {
        let mut min_bit = None;
        let mut mask = 0 as i64;

        zero = true;
        for (count, num) in goal_vec.iter().enumerate() {
            if *num != 0{
                zero = false;
                mask += (1 as i64) << count;

                if min_bit.is_none() || *num < goal_vec[min_bit.unwrap()] {
                    min_bit = Some(count);
                }
            }
        }
        if zero {
            break;
        }

        let count = goal_vec[min_bit.unwrap() as usize];
        let buttons = min_buttons_for_mask(mask, nums).unwrap();
        for button in buttons.into_iter() {
            list[button] += count as usize;
            print!("{} ", button);
        }
        for (index, num) in goal_vec.iter_mut().enumerate() {
            if mask & (1 as i64) << index != 0 {
                *num -= count as i64;
            }
        }
        println!("Count: {} Goal_vec: {:?}", count, goal_vec);
    }
    
    //Need to synthesize: If a number A is equivalent to two or more numbers that are used, can replace
    //the numbers with A.

    let mut total = 0 as usize;
    for num in list.iter() {
        total += *num;
    }
    return Some(total);
}

//Really slow solution...
pub fn sum_slice(goal_vec: &mut Vec<i64>, nums_vec: &[i64]) -> Option<usize> {
    let mut min = Some(0);
    for num in goal_vec.iter() {
        if *num != 0 {
            min = None;
        }
    }
    if min.is_none() {
        //A stack of (index, count) pairs, where index is the index of the number in nums_vec that we are currently trying to add to the sum, 
        //and count is the number min_lenof times we have added that number to the sum.
        let mut stack = Vec::<(usize,usize)>::new(); 
        let mut next_index = 0;
        let mut total_presses = 0; //The total number of numbers that have been subtracted from goal_vec so far.
        while next_index < nums_vec.len() || total_presses > 0 {
            let mut min_bit = usize::MAX;
            let num = nums_vec[next_index];


            // for (n, pair) in stack.iter().enumerate() {
            //     let val = pair.0;
            //     let ct = pair.1;

            //     let x = ct + val;
            // }
            
            for (ct, goal) in goal_vec.iter_mut().enumerate() {
                let bit_value = (num >> ct) & 1;
                if bit_value == 1{
                    if (*goal as usize) < min_bit {
                        min_bit = *goal as usize;
                    }
                }
            }
            
            let mut potential_min = None;
            if min_bit > 0 && min_bit != usize::MAX && (min.is_none() || total_presses + min_bit < min.unwrap()) {
                total_presses += min_bit;
                potential_min = Some(total_presses);
                for (ct, goal) in goal_vec.iter_mut().enumerate() {
                    let old_value = *goal;
                    let bit_value = (num >> ct) & 1;
                    let new_value = old_value - (bit_value * min_bit as i64);
                    
                    *goal = new_value;

                    if new_value != 0 {
                        potential_min = None;
                    }
                }
                stack.push((next_index, min_bit));
            }

            if potential_min.is_some() && (min.is_none() || potential_min.unwrap() < min.unwrap()) {
                min = potential_min;
            }
            else {
                next_index += 1;
            }

            //Want to pop elements off the stack until we can add a new number to the sum, or until the stack is empty.
            if potential_min.is_some() || next_index == nums_vec.len() {
                while total_presses > 0 {
                    let stack_len = stack.len();
                    let (back, count) = stack[stack_len - 1];
                    let num = nums_vec[back];

                    let mut coeff = 1 as usize;
                    if back == nums_vec.len() - 1 {
                        coeff = count;
                    }
                    for (ct, goal) in goal_vec.iter_mut().enumerate() {
                        *goal += (((num >> ct) & 1) as usize * coeff) as i64;
                    }
                    total_presses -= coeff;
                    if count - coeff == 0 {
                        stack.pop();
                        next_index = back + 1;
                    }
                    else {
                        stack[stack_len - 1] = (back, count - coeff);
                        next_index = back + 1;
                    }
                    if next_index < nums_vec.len() {
                        break;
                    }
                }   
            }
        }
    }
    return min;
}


//This approach won't work.
pub fn problem_two_naive(filename: &str) -> usize{
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

            let result = sum_slice3(&mut goal_vec, &nums_vec).unwrap_or(19090909);
            total += result;
            println!("Result: {}", result);
        }
    }
    
    return total;
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    return n;
}

pub fn gaussian_elimination(matrix: &mut Vec<Vec<(i64,i64)>>) {
    let num_counters = matrix.len();
    let num_buttons = matrix[0].len() - 1;

    let mut leftmost_nonzero = 0;
    for row in 0..num_counters {
        let mut row_count = 0;
        let mut found = false;
        while !found && leftmost_nonzero < num_buttons{
            for row2 in row..num_counters {
                if matrix[row2][leftmost_nonzero].0 != 0 {
                    row_count = row2;
                    found = true;
                    break;
                }
            }
            if !found {
                leftmost_nonzero += 1;
            }
        }

        if leftmost_nonzero >= num_buttons {
            break;
        }

        if row_count != row {
            matrix.swap(row, row_count);
        }
        let mut coeff = (matrix[row][leftmost_nonzero].1, matrix[row][leftmost_nonzero].0);
        if coeff.1 < 0 {
            coeff.0 *= -1;
            coeff.1 *= -1;
        }
        if coeff.0 != 1 || coeff.1 != 1 {
            for col in leftmost_nonzero..num_buttons + 1 {
                matrix[row][col].0 *= coeff.0;
                matrix[row][col].1 *= coeff.1;
                if matrix[row][col].0 == 0 {
                    matrix[row][col].1 = 1;
                }
                else if matrix[row][col].1 != 1 {
                    let cd = gcd(matrix[row][col].0.abs() as u64, matrix[row][col].1.abs() as u64) as i64;
                    matrix[row][col].0 /= cd;
                    matrix[row][col].1 /= cd;
                }
            }
        }
        assert!(matrix[row][leftmost_nonzero].0 == 1 && matrix[row][leftmost_nonzero].1 == 1);
        for row2 in 0..num_counters {
            if row2 != row {
                let coeff = matrix[row2][leftmost_nonzero];
                if coeff.0 != 0 {
                    for col in leftmost_nonzero..num_buttons + 1 {
                        let cell = matrix[row][col];
                        let subtracted_amount = (coeff.0 * cell.0, coeff.1 * cell.1);

                        matrix[row2][col].0 = matrix[row2][col].0 * subtracted_amount.1 - subtracted_amount.0 * matrix[row2][col].1;
                        matrix[row2][col].1 *= subtracted_amount.1;
                        if matrix[row2][col].0 == 0{
                            matrix[row2][col].1 = 1;
                        }                               
                        else if matrix[row2][col].1 != 1 {
                            let cd = gcd(matrix[row2][col].0.abs() as u64,  matrix[row2][col].1.abs() as u64) as i64;
                            matrix[row2][col].0 /= cd;
                            matrix[row2][col].1 /= cd;
                        }
                    }
                }
            }
        }
        leftmost_nonzero += 1;
        //print_matrix(matrix);
    }
}

pub fn brute_force(matrix: &Vec<Vec<(i64,i64)>>, bounds : Vec<usize>) -> usize {
    let num_counters = matrix.len();
    let num_buttons = matrix[0].len() - 1;


    //Find each free variable
    let mut free_values = Vec::<(usize,usize)>::new();
    for col in 0..num_buttons {
        let mut count = 0;
        for row in 0..num_counters {
            if matrix[row][col].0 != 0 {
                count += 1;
            }
        }
        if count > 1 {
            free_values.push((col,0));
        }
    }
    
    if free_values.len() == 0{
        let mut total = 0;
        for row in 0..num_counters {
            total += matrix[row][num_buttons].0 as usize;
            assert!(matrix[row][num_buttons].0 >= 0);
        }
        return total;
    }
    else{
        //Now we can brute force over all possible values of the free variables...
        let mut min_total = usize::MAX;
        let mut min_settings = Vec::<usize>::new();

        while free_values[0].1 <= bounds[free_values[0].0] {
            //println!("Trying free values: {:?}", free_values);
            let mut potential_settings = Vec::<usize>::new();
            for _ in bounds.iter() {
                potential_settings.push(0);
            }
            //println!("Trying free values: {:?}", free_values);
            //Check that the solution is valid and the total of variables is less than min total.
            let mut variable_total = 0;
            for (col, value) in free_values.iter(){
                variable_total += *value;
                potential_settings[*col] = *value;
            }
            let mut viable = true;
            for row in 0 .. num_counters {
                let mut goal_sum = matrix[row][num_buttons];
                for (col, value) in free_values.iter(){
                    if matrix[row][*col].0 == 0 {
                        continue;
                    }
                    let matrix_cell = matrix[row][*col];
                    goal_sum.0 = goal_sum.0 * matrix[row][*col].1 - matrix[row][*col].0 * (*value as i64) * goal_sum.1;
                    goal_sum.1 *= matrix[row][*col].1;

                    if goal_sum.0 != 0 {
                        let cd = gcd(goal_sum.0.abs() as u64,  goal_sum.1.abs() as u64) as i64;
                        assert!(goal_sum.0 % cd == 0 && goal_sum.1  % cd == 0); 
                        goal_sum.0 /= cd;
                        goal_sum.1 /= cd;
                    }
                    else {
                        goal_sum.1 = 1;
                    }
                }

                if goal_sum.1 != 1 || goal_sum.0 < 0 {
                    viable = false;
                    break;
                }
                else{
                    variable_total += goal_sum.0 as usize;
                    for col in 0..num_buttons {
                        if matrix[row][col].0 != 0 {
                            potential_settings[col] = goal_sum.0 as usize;
                            break;
                        }
                    }
                }
            }

            if viable && variable_total < min_total {
                min_total = variable_total;
                min_settings = potential_settings;
            }



            //Update free values to try the next possible solution.
            let mut cur = free_values.len() - 1;
            free_values[cur].1 += 1;
            while free_values[cur].1 > bounds[free_values[cur].0] {
                if cur == 0 {
                    break;
                }
                free_values[cur].1 = 0;
                cur -= 1;
                free_values[cur].1 += 1;
            }
        }
        println!("Min settings: {:?}", min_settings);
        //assert!(min_total != usize::MAX);
        return min_total;
    }
}

pub fn print_matrix(matrix: &Vec<Vec<(i64,i64)>>) -> bool{
    let mut bugged = false;
    let num_cols = matrix[0].len();
    for row in matrix.iter() {
        println!("{:?}", row);
        if !bugged && row[num_cols - 1].0 != 0 {
            bugged = true;
            for col in 0..num_cols - 1 {
                if row[col].0 != 0 {
                    bugged = false;
                    break;
                }
            }
        }
    }
    println!();
    return bugged;
}

pub fn find_minimum_solution_total(matrix: &mut Vec<Vec<(i64,i64)>>, count: usize) -> usize {
    let num_counters = matrix.len();
    let num_buttons = matrix[0].len() - 1;


    let mut bounds = Vec::<usize>::new();
    for col in 0..num_buttons {
        let mut bound = usize::MAX;
        for row in 0..num_counters {
            if matrix[row][col].0 > 0 && (matrix[row][num_buttons].0 as usize) < bound {
                bound = matrix[row][num_buttons].0 as usize;
            }
        }
        bounds.push(bound);
    }
    println!("Bounds: {:?}", bounds);
    println!("Number: {}", count);
    print_matrix(matrix);

    gaussian_elimination(matrix);


    assert!(!print_matrix(matrix));
    return brute_force(matrix, bounds);
    //print_matrix(matrix);
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
    for (count, line) in all_lines.iter().enumerate() {
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

            //Create a matrix that defines the system of linear equations
            //The last column of the matrix contains the values that each counter needs to be incremented to.
            //The other columns of the matrix correspond to buttons. Each cell in a column for a given row 
            //indicates whether pressing the button will increment the counter for that row.
            let mut matrix = Vec::<Vec<(i64,i64)>>::new();
            matrix.reserve(goal_vec.len());
            for (count, num) in goal_vec.iter().enumerate() {
                let mut row = Vec::<(i64,i64)>::new();
                row.reserve(nums_vec.len() + 1);
                for num2 in nums_vec.iter() {
                    if *num2 & ((1 as i64) << count) != 0 {
                        row.push((1,1));
                    } else {
                        row.push((0,1));
                    }
                } 
                row.push((*num,1));
                matrix.push(row);
            }

            let result = find_minimum_solution_total(&mut matrix, count);
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
