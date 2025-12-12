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
    let mut _total: i64 = 0;
    return _total;
}
pub fn problem_one(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let mut all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    let mut _total: i64 = 0;

    let mut position_list = Vec::<(i64,i64,i64,usize)>::new();
    for (count,line) in all_lines.iter().enumerate() {
        let tokens:Vec<&str> = line.split(",").collect();
        if tokens.len() == 3 {
            let x = tokens[0].to_string().trim().parse::<i64>().unwrap();
            let y = tokens[1].to_string().trim().parse::<i64>().unwrap();
            let z = tokens[2].to_string().trim().parse::<i64>().unwrap();

            position_list.push((x,y,z,count));
        }
    }

    let mut distance_pairs = Vec::<(i64,usize,usize)>::new();
    for (i, (aX,aY,aZ,_)) in position_list.iter().enumerate() {
        for j in (i + 1)..position_list.len() {
            let (bX, bY, bZ,_) = position_list[j];

            let distX: i64 = bX - aX;
            let distY: i64 = bY - aY;
            let distZ: i64 = bZ - aZ;
            let dist_sq: i64 = distX * distX + distY * distY + distZ * distZ;
            let distance = f64::sqrt(dist_sq as f64) * 10000.0;

            distance_pairs.push((distance as i64,i,j));
        }
    }
    distance_pairs.sort();

    for (distance, i1, i2) in distance_pairs.enumerate() {
        println!("{} {} {}",distance,i1,i2);
        let new_team = if i1 < i2 { i1 } else { i2 };
        
    }
    return _total;
}
fn main() {
    let result = problem_one("input2.txt");
    println!("{}",result);
}
