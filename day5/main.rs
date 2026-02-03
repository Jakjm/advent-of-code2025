#![feature(btree_cursors)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::{BTreeMap};

use std::ops::Bound;


pub fn problem_one_nightly(filename: &str) -> i64{
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

            //Search for a range whose start is the furthest to the right but still <= range_start
            let mut cursor = ranges.upper_bound_mut( Bound::Included(&range_start) );
            let mut key_value_opt = cursor.peek_prev();

            if let Some((key,value)) = key_value_opt {
                //If value >= range_start, we have an overlap
                if *value >= range_start {
                    range_start = *key;
                    if *value > range_end 
                    {
                        range_end = *value;
                    }
                    cursor.remove_prev();
                }
                key_value_opt = cursor.next();
                while let Some((key,value)) = key_value_opt && *key <= range_end && *value >= range_start {
                    if *value > range_end 
                    {
                        range_end = *value;
                    }
                    cursor.remove_prev();       
                    key_value_opt = cursor.next();
                }
            }
            ranges.insert(range_start, range_end);
        }
        else if tokens.len() == 1 {
            let token = tokens[0].to_string().trim().parse::<i64>().unwrap();

            //Iterator to largest pair with start <= token
            match ranges.range(..=token).rev().next(){
                Some((_key, value)) => {
                    if *value >= token {
                        total += 1;
                    }
                }
                None => {}
            }
        }
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
                        //println!("{}", token);
                        total += 1;
                    }
                }
                None => {}
            }
        }
    }
    return total;
}

pub fn problem_two_nightly(filename: &str) -> i64{
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

            //Search for the gap before the smallest key that is greater than range_end
            let mut cursor = ranges.lower_bound_mut( Bound::Excluded(&range_end) );
            let mut key_value_opt = cursor.prev();
            while let Some((key,value)) = key_value_opt && *key <= range_end && *value >= range_start {
                if *value > range_end {
                    range_end = *value;
                }
                if *key < range_start {
                    range_start = *key;
                }
                total -= (*value - *key) + 1;
                cursor.remove_next();
                key_value_opt = cursor.prev();
            }
            total += (range_end - range_start) as i64 + 1 as i64;
            ranges.insert(range_start, range_end);
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
    // let result = problem_one_nightly("input.txt");
    // let result_original = problem_one("input.txt");
    // println!("{} {}",result, result_original);


    let result = problem_two_nightly("input.txt");
    let result_original = problem_two("input.txt");
    println!("{} {}",result, result_original);
}
