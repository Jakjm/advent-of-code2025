use std::fs::File;
use std::path::Path;
use std::io::{BufRead,BufReader};
pub fn problem_two(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let mut reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    //The best approach to solving this would probably be to do a first pass over all lines, 
    //during which I can calculate the cell of maximum width for each column.

    //After this, I can then split each line into cells, each cell having the 
    //exact width of the maximum width of its column.

    //Store a vector containing sums/products of the values in each column
    //let mut nums = Vec::<Vec<i64>>::new();
    let mut _total: i64 = 0;
    for line in all_lines{
        let tokens:Vec<&str> = line.split_whitespace().collect();
        let first_token = tokens[0].trim();

        for token in tokens.iter(){
            print!("`{}` ", token);
        }
        println!();
    }
    return _total;
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
                if *token == "+" {
                    total += *sum;
                }
                else{
                    total += *product;
                }
            }
        }
    }
    return total;
}
fn main() {
    let result = problem_two("input2.txt");
    println!("{}",result);
}
