use std::fs::File;
use std::path::Path;
use std::io::{BufRead,BufReader};
use std::convert::TryInto;


pub fn lame_heuristic(width: usize, height: usize, amounts: Vec<usize>, shapes: &Vec::<([[u8; 3]; 3], usize)>) -> usize {
    let area = width * height;

    let mut total_sqs = 0;
    let mut total_shapes = 0;
    let num_three_by_three_sqs = (width / 3) * (height / 3);
    for (index, amount) in amounts.iter().enumerate() {
        total_sqs += amount * shapes[index].1;
        total_shapes += amount;
    }
    if total_sqs > area {
        return 0;
    }
    else if total_shapes <= num_three_by_three_sqs {
        return 1;
    }
    else {
        assert!(false);
        return 0;
    }
}

pub fn problem_one(filename: &str) -> usize{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return 0; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();


    let mut total = 0;

    //Build the graph using a map of sources to the number of paths to the source from "you" and the list of destinations.
    let mut shapes = Vec::<([[u8; 3]; 3], usize)>::new();
    let mut shape_line = -1 as isize;
    let mut cur_shape : [[u8; 3]; 3] = [[0, 0, 0],[0, 0, 0],[0, 0, 0]];
    let mut shape_sq_count = 0;
    for line in all_lines.iter() {
        if shape_line >= 0 {
            let cur_line = line.as_bytes()[..3].try_into().unwrap();
            for c in cur_line {
                if c == b'#' {
                    shape_sq_count += 1;
                }
            }
            cur_shape[shape_line as usize] = cur_line;
            if shape_line < 2 {
                shape_line += 1;
            }
            else{
                shapes.push((cur_shape, shape_sq_count));
                shape_line = -1;
                shape_sq_count = 0;
            }
        }
        else if line.len() > 1{
            if line.as_bytes()[1] == b':' {
                shape_line = 0;
            }
            else{
                let tokens:Vec<&str> = line.split(" ").collect();
                let first_token = tokens[0];
                let (width_part, first_token) = first_token.split_at(2);
                let (_, first_token) = first_token.split_at(1);
                let (height_part, _) = first_token.split_at(2);
                let width = width_part.parse::<usize>().unwrap();
                let height = height_part.parse::<usize>().unwrap();

                let mut amounts = Vec::<usize>::new();
                amounts.reserve(tokens.len() - 1);
                for i in 1..tokens.len(){
                    let num = tokens[i].trim().parse::<usize>().unwrap();
                    amounts.push(num);
                }
                total += lame_heuristic(width, height, amounts, &shapes);
            }
        }
    }

    for (count, shape) in shapes.iter().enumerate() {
        println!("{}: {}", count, shape.1);
        for i in 0..3 {
            println!("{:?}", shape.0[i] );
        }
        println!();
    }
    //Note that by the description, it is a directed acyclic graph, which makes the structure of the dfs much simpler.
    return total;
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    let result = problem_one("input.txt");
    println!("{}",result);
}
