use std::fs::File;
use std::path::Path;
use std::io::{BufRead,BufReader};
//use std::collections::{BTreeMap};
pub fn problem_two(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let _all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();
    return 0;
}
pub fn problem_one(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();


    let mut other_pts = Vec::<(i64,i64)>::new();
    let mut max_area : i64 = -1;
    for line in all_lines.iter() {
        let tokens:Vec<&str> = line.split(",").collect();
        if tokens.len() == 2 {
            let x = tokens[0].to_string().trim().parse::<i64>().unwrap();
            let y = tokens[1].to_string().trim().parse::<i64>().unwrap();

            for (o_x,o_y) in other_pts.iter() {
                let (diff_x, diff_y) = (o_x - x,o_y - y);
                let area = (i64::abs(diff_x) + 1) * (i64::abs(diff_y) + 1);
                if area > max_area {
                    max_area = area;
                }
            }
            other_pts.push((x,y));
        }
    }

    
    return max_area;
}
fn main() {
    let result = problem_one("input.txt");
    println!("{}",result);
}
