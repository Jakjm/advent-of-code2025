use std::fs::File;
use std::path::Path;
use std::io::{BufRead,BufReader};
pub fn problem_two(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();
    let line_len = all_lines[0].len();
    let mut line_sums : Vec<i64> = Vec::<i64>::with_capacity(line_len);
    line_sums.resize(line_len,0);
   
    for (index,ch) in all_lines[0].chars().into_iter().enumerate() {
        if ch == 'S' {
            line_sums[index] += 1;
        }
    }
    for prev_cur_line in all_lines.windows(2) {
        match prev_cur_line{
            [prev_line, line] => {
                for index in 0..line.len() {
                    let prev_slice = &prev_line[index..index + 1];
                    let cur_slice = &line[index..index + 1];
                    if cur_slice == "^" && line_sums[index] > 0 {
                        let num_split = line_sums[index];
                        line_sums[index] = 0;
                        
                        if index > 0 && &line[index-1..index] == "." {
                            line_sums[index - 1] += num_split;
                            // unsafe {
                            //     line.as_bytes_mut()[index - 1] = b'|';
                            // }
                        }
                        if index + 1 < line.len() && &line[index+1..index+2] == "." {
                            line_sums[index + 1] += num_split;
                            // unsafe {
                            //     line.as_bytes_mut()[index + 1] = b'|'
                            // }
                        }
                    }
                }
                println!("{}", line);
            },
            _ => {

            }
        }
    }
    let mut total: i64 = 0;
    for val in line_sums {
        total += val;
    }
    return total;
}
pub fn problem_one(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let mut all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();
    let mut total : i64 = 0;

    //Iterate over pairs of consecutive lines in the array...
    println!("{}",all_lines[0]);
    for line_no in 1.. all_lines.len(){
        let prev_cur_line = all_lines.get_mut(line_no-1..line_no+1).unwrap();
        match prev_cur_line {
            [prev_line, line] => {
                for index in 0..line.len() {
                    let prev_slice = &prev_line[index..index + 1];
                    let cur_slice = &line[index..index + 1];
                    if prev_slice == "|" || prev_slice == "S" {
                        if cur_slice == "." {
                            unsafe {
                                line.as_bytes_mut()[index] = b'|';
                            }
                        }
                        else if cur_slice == "^" {
                            let mut split = false;
                            if index > 0 && &line[index-1..index] == "." {
                                unsafe {
                                    line.as_bytes_mut()[index - 1] = b'|';
                                }
                                split = true;
                            }
                            if index + 1 < line.len() && &line[index+1..index+2] == "." {
                                unsafe {
                                    line.as_bytes_mut()[index + 1] = b'|'
                                }
                                split = true;
                            }
                            if split {
                                total += 1;
                            }
                        }
                    }
                }
                println!("{}", line);
            },
            _ => {

            }
        }
    }
    return total;
}
fn main() {
    let result = problem_two("input.txt");
    println!("{}",result);
}
