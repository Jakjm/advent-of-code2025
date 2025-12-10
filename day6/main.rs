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

    let mut column_widths = Vec::<usize>::new();
    //The best approach to solving this would probably be to do a first pass over all lines, 
    //during which I can calculate the cell of maximum width for each column.

    //After this, I can then split each line into cells, each cell having the 
    //exact width of the maximum width of its column.

    //Store a vector containing sums/products of the values in each column
    for line in all_lines.iter() {
        let tokens:Vec<&str> = line.split_whitespace().collect();
        if column_widths.len() < tokens.len(){
            column_widths.resize(tokens.len(),0);
        }
        for (token,column_width) in tokens.into_iter().zip(column_widths.iter_mut()) {
            if token.len() > *column_width {
                *column_width = token.len();
            }
        }
    }
    
    let mut total: i64 = 0;
    //Contains a vector of the numbers for each column.
    let mut nums = Vec::<Vec<i64>>::with_capacity(column_widths.len());
    for column_width in column_widths.iter() {
        //Each vector has as many numbers as there are digits in the width...
        let mut num_vec = Vec::<i64>::with_capacity(*column_width);
        num_vec.resize(*column_width, 0);
        nums.push(num_vec);
    }
    //Iterate over each line...
    for (line_no,line) in all_lines.iter().enumerate(){
        let mut rest = line.clone();
        print!("Line {}: ",line_no);
        for (width,num_vec) in column_widths.iter().zip(nums.iter_mut())
        {
            let (cell,mut right) = rest.split_at_mut(*width);
            let trimmed = cell.trim();
            print!("{} ", cell);
            if trimmed == "*" || trimmed == "+" {
                let (mut product, mut sum) : (i64,i64) = (1,0);
                for v in num_vec.iter() {
                    print!("{} ", v);
                    product *= v;
                    sum += *v;
                }
                total += if trimmed == "*" { product } else { sum };
                println!("({})",if trimmed == "*" { product } else { sum });
            }
            else{
                for (ch,v) in cell.chars().rev().zip(num_vec.iter_mut()) {
                    if '0' as i64 <= ch as i64 && ch as i64 <= '9' as i64 {
                        let digit = ch as i64 - '0' as i64;
                        *v = (*v * 10) + digit;
                    }
                }
            }
            //Trim extra space before starting next line.
            if right.starts_with(" "){
                (_, right) = right.split_at_mut(1);
            }
            rest = right.to_string();
        }
        println!();
    }
    return total;
}
pub fn problem_one(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let mut reader = BufReader::new(file_result.unwrap());
    let lines = (&mut reader).lines();

    //Store a vector containing sums/products of the values in each column
    let mut results = Vec::<(i64,i64)>::new();
    let mut total: i64 = 0;
    for line in lines.map_while(Result::ok){
        let tokens:Vec<&str> = line.split_whitespace().collect();
        let first_token = tokens[0].trim();

        if first_token != "+" && first_token != "*" {
            if results.is_empty(){
                for token in tokens.iter(){
                    let parsed_token = token.to_string().trim().parse::<i64>().unwrap();
                    results.push((parsed_token,parsed_token));
                }
            }
            else{
                for (token,(sum,product)) in tokens.iter().zip(results.iter_mut()){
                    let parsed_token = token.to_string().trim().parse::<i64>().unwrap();
                    *sum += parsed_token;
                    *product *= parsed_token;
                }
            }
        }
        else{
            for (token,(sum,product)) in tokens.iter().zip(results.iter()){
                total += if *token == "+" { *sum } else { *product };
            }
        }
    }
    return total;
}
fn main() {
    let result = problem_two("input.txt");
    println!("{}",result);
}
