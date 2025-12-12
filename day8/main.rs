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
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    let mut position_circuit_list = Vec::<(i64,i64,i64,usize)>::new();
    for (circuit_no,line) in all_lines.iter().enumerate() {
        let tokens:Vec<&str> = line.split(",").collect();
        if tokens.len() == 3 {
            let x = tokens[0].to_string().trim().parse::<i64>().unwrap();
            let y = tokens[1].to_string().trim().parse::<i64>().unwrap();
            let z = tokens[2].to_string().trim().parse::<i64>().unwrap();

            position_circuit_list.push((x,y,z,circuit_no));
        }
    }

    //Get a list of pairs of junction boxes and the straight line distance between them,
    //Sorted by distance.
    let mut distance_pairs = Vec::<(i64,usize,usize)>::new();
    for (i, (a_x,a_y,a_z,_)) in position_circuit_list.iter().enumerate() {
        for j in (i + 1)..position_circuit_list.len() {
            let (b_x, b_y, b_z,_) = position_circuit_list[j];

            let dist_x: i64 = b_x - a_x;
            let dist_y: i64 = b_y - a_y;
            let dist_z: i64 = b_z - a_z;
            let dist_sq: i64 = dist_x * dist_x + dist_y * dist_y + dist_z * dist_z;
            let distance = f64::sqrt(dist_sq as f64) * 10000.0;

            distance_pairs.push((distance as i64,i,j));
        }
    }
    distance_pairs.sort();

    let (mut li1, mut li2) = (0,0);
    for (_, i1, i2) in distance_pairs.iter() {
        let (_,_,_,circuit_a) = position_circuit_list[*i1];
        let (_,_,_,circuit_b) = position_circuit_list[*i2];
        if circuit_a == circuit_b { 
            continue;
        }
        (li1,li2) = (*i1,*i2);
        let (new_team,old_team) = if circuit_a < circuit_b {
            (circuit_a,circuit_b)
        }
        else {
            (circuit_b,circuit_a)
        };
        for (_,_,_,circuit) in position_circuit_list.iter_mut(){
            if *circuit == old_team {
                *circuit = new_team;
            }
        }
    }

    let (a_x,_,_,_) = position_circuit_list[li1];
    let (b_x,_,_,_) = position_circuit_list[li2];
    return a_x * b_x;
}
pub fn problem_one(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    let mut position_circuit_list = Vec::<(i64,i64,i64,usize)>::new();
    for (circuit_no,line) in all_lines.iter().enumerate() {
        let tokens:Vec<&str> = line.split(",").collect();
        if tokens.len() == 3 {
            let x = tokens[0].to_string().trim().parse::<i64>().unwrap();
            let y = tokens[1].to_string().trim().parse::<i64>().unwrap();
            let z = tokens[2].to_string().trim().parse::<i64>().unwrap();

            position_circuit_list.push((x,y,z,circuit_no));
        }
    }

    //Get a list of pairs of junction boxes and the straight line distance between them,
    //Sorted by distance.
    let mut distance_pairs = Vec::<(i64,usize,usize)>::new();
    for (i, (a_x,a_y,a_z,_)) in position_circuit_list.iter().enumerate() {
        for j in (i + 1)..position_circuit_list.len() {
            let (b_x, b_y, b_z,_) = position_circuit_list[j];

            let dist_x: i64 = b_x - a_x;
            let dist_y: i64 = b_y - a_y;
            let dist_z: i64 = b_z - a_z;
            let dist_sq: i64 = dist_x * dist_x + dist_y * dist_y + dist_z * dist_z;
            let distance = f64::sqrt(dist_sq as f64) * 10000.0;

            distance_pairs.push((distance as i64,i,j));
        }
    }
    distance_pairs.sort();

    for (index, (_, i1, i2)) in distance_pairs.iter().enumerate() {
        let (_,_,_,circuit_a) = position_circuit_list[*i1];
        let (_,_,_,circuit_b) = position_circuit_list[*i2];
        if index >= 1000 { //Stop after the first 1000 connections...
            break;
        }
        if circuit_a == circuit_b { //Junction boxes already connected.
            continue;
        }
        let (new_team,old_team) = if circuit_a < circuit_b {
            (circuit_a,circuit_b)
        }
        else {
            (circuit_b,circuit_a)
        };
        for (_,_,_,circuit) in position_circuit_list.iter_mut(){
            if *circuit == old_team {
                *circuit = new_team;
            }
        }
    }

    let mut circuit_count = Vec::<i64>::with_capacity(position_circuit_list.len());
    circuit_count.resize(position_circuit_list.len(),0);

    let (mut max_one,mut max_two,mut max_three) : (i64, i64, i64) = (0,0,0);
    for (_,_,_,circuit) in position_circuit_list {
        circuit_count[circuit] += 1;

        if circuit_count[circuit] > max_one {
            max_three = max_two;
            max_two = max_one;
            max_one = circuit_count[circuit];
        }
        else if circuit_count[circuit] > max_two {
            max_three = max_two;
            max_two = circuit_count[circuit];
        }
        else if circuit_count[circuit] > max_three {
            max_three = circuit_count[circuit];
        }
    }

    println!("{} {} {}", max_one, max_two, max_three);
    return max_one * max_two * max_three;
}
fn main() {
    let result = problem_two("input.txt");
    println!("{}",result);
}
