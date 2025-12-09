use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::{BTreeMap};

pub fn problem_one(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let mut reader = BufReader::new(file_result.unwrap());
    let lines = (&mut reader).lines();

    let mut ranges = BTreeMap::<i64,i64>::new(); 
    let mut total: i64 = 0;
    for line in lines.map_while(Result::ok){
        let tokens:Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 2 {
            let mut range_start = tokens[0].to_string().trim().parse::<i64>().unwrap();
            let mut range_end = tokens[1].to_string().trim().parse::<i64>().unwrap();

            //Remove all overlapping ranges from the map, 
            //And create a new range that extends to the min/max of all of them.
            //Not the most efficient way to do this but it's okay.
            ranges.retain(|key,value| {
                if *key <= range_end && range_start <= *value {
                    if *key < range_start {
                        range_start = *key;
                    }
                    if *value > range_end {
                        range_end = *value;
                    }
                    return false;
                }
                true
            });
            ranges.insert(range_start, range_end);
        }
        else if tokens.len() == 1 {
            let token = tokens[0].to_string().trim().parse::<i64>().unwrap();

            //Iterator to largest pair with start <= token
            match ranges.range(..=token).rev().next(){
                Some((_key, value)) => {
                    if *value >= token {
                        println!("{}", token);
                        total += 1;
                    }
                }
                None => {}
            }
        }
    }
    return total;
}
pub fn problem_two(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1;
    }
    let mut reader = BufReader::new(file_result.unwrap());
    let lines = (&mut reader).lines();

    let mut ranges = BTreeMap::<i64,i64>::new(); 
    let mut total: i64 = 0;
    for line in lines.map_while(Result::ok){
        let tokens:Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 2 {
            let mut range_start = tokens[0].to_string().trim().parse::<i64>().unwrap();
            let mut range_end = tokens[1].to_string().trim().parse::<i64>().unwrap();

            //Remove all overlapping ranges from the map, 
            //And create a new range that extends to the min/max of all of them.
            //Not the most efficient way to do this but it's okay.
            ranges.retain(|key,value| {
                if *key <= range_end && range_start <= *value {
                    if *key < range_start {
                        range_start = *key;
                    }
                    if *value > range_end {
                        range_end = *value;
                    }
                    total -= (*value - *key) + 1;
                    return false;
                }
                true
            });
            ranges.insert(range_start, range_end);
            total += (range_end - range_start) + 1;
        }
    }
    return total;
}
fn main() {
    let result = problem_two("input.txt");
    println!("{}",result);
}
